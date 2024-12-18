/*
Entities:
  - floor tile
  - wall tile
  - robot
  - box
*/
use crate::graph::{Coordinate, Direction, Index, Matrix};
use std::borrow::Cow;
use std::collections::{HashSet, VecDeque};
use std::mem;

type Warehouse = Vec<Vec<char>>;
type Instructions = Vec<Direction>;

const BOX: char = 'O';
const EMPTY: char = '.';
const ROBOT: char = '@';
const WALL: char = '#';

fn parse_instruction(instruction: char) -> Direction {
    match instruction {
        '^' => Direction::North,
        '>' => Direction::East,
        '<' => Direction::West,
        'v' => Direction::South,
        ch @ _ => panic!("Invalid instruction {}", ch),
    }
}

fn parse_input(input: &str) -> (Warehouse, Instructions) {
    let (warehouse, instructions) = input.split_once("\n\n").unwrap();

    let warehouse = warehouse.lines().map(|l| l.chars().collect()).collect();

    let instructions = instructions
        .chars()
        .filter_map(|ch| {
            if ch == '\n' {
                None
            } else {
                Some(parse_instruction(ch))
            }
        })
        .collect();
    (warehouse, instructions)
}

fn update_warehouse(warehouse: &mut Warehouse, instructions: &Instructions) {
    let Some(mut robot_location) = find_robot(warehouse) else {
        return;
    };
    instructions.iter().for_each(|instruction| {
        if let Cow::Owned(new_location) = try_move(warehouse, &robot_location, &instruction) {
            robot_location = new_location;
        }
    })
}
fn update_wide_warehouse(warehouse: &mut Warehouse, instructions: &Instructions) {
    let Some(mut robot_location) = find_robot(warehouse) else {
        return;
    };
    instructions.iter().for_each(|instruction| {
        // println!("moving {:?}", &instruction);
        if let Cow::Owned(new_location) = try_wide_move(warehouse, &robot_location, &instruction) {
            robot_location = new_location;
        }
        // print_warehouse(&warehouse);
        // println!();
    })
}

fn try_wide_move<'a>(
    warehouse: &mut Warehouse,
    robot_location: &'a Coordinate,
    direction: &Direction,
) -> Cow<'a, Coordinate> {
    // first step, check whether there's a box ahead of the robot
    // north south is different from west east

    match direction {
        Direction::North | Direction::South => {
            // this one isn't fun
            // we need to keep track of every thing that can move forward
            let mut bfs_queue = VecDeque::new();
            bfs_queue.push_back(robot_location.clone());
            let mut to_shift = Vec::new();

            while let Some(last_point) = bfs_queue.pop_front() {
                let ahead_point = last_point.shift(direction).unwrap();
                let ahead_tile = warehouse
                    .get_scalar(&ahead_point.try_into().unwrap())
                    .unwrap();

                match ahead_tile {
                    '#' => return Cow::Borrowed(robot_location),
                    '[' => {
                        let adjacent_point = ahead_point
                            .shift(&Direction::East).unwrap();
                        bfs_queue.push_back(ahead_point);
                        bfs_queue.push_back(adjacent_point);
                        to_shift.push(last_point);
                    }
                    ']' => {
                        let adjacent_point = ahead_point
                            .shift(&Direction::West).unwrap();

                        if bfs_queue.back() != Some(&ahead_point) {
                            bfs_queue.push_back(adjacent_point);
                            bfs_queue.push_back(ahead_point);
                        }
                        to_shift.push(last_point);
                    }
                    '.' => {
                        to_shift.push(last_point);
                        continue
                    },
                    ch@_ => panic!("Invalid tile: {}", ch),
                }
            }
            println!();

            // now we have the walls, we just need to update them all (FUCK).
            to_shift.into_iter().rev().for_each(|p| {
                let last_tile =warehouse.get_scalar(&p.try_into().unwrap()).unwrap();
                let new_position = p.shift(&direction).unwrap();
                warehouse.set_scalar(&new_position.try_into().unwrap(), *last_tile);
                warehouse.set_scalar(&p.try_into().unwrap(), '.');
                // print_warehouse(&warehouse);

            });
            warehouse.set_scalar(&robot_location.try_into().unwrap(), '.');
            Cow::Owned(robot_location.shift(&direction).unwrap())
        }
        Direction::West | Direction::East => {
            let mut walls_between = Vec::new();
            let mut next_coordinate = robot_location.shift(&direction).unwrap();
            let mut next_tile = warehouse
                .get_scalar(&next_coordinate.try_into().unwrap())
                .unwrap();
            loop {
                match next_tile {
                    '#' => {
                        return Cow::Borrowed(robot_location);
                    }
                    '.' => {
                        let direction = direction.get_opposite();
                        while let Some(next_tile) = walls_between.pop() {
                            warehouse.set_scalar(&next_coordinate.try_into().unwrap(), next_tile);
                            next_coordinate = next_coordinate.shift(&direction).unwrap()
                        }
                        warehouse.set_scalar(&next_coordinate.try_into().unwrap(), ROBOT);
                        warehouse.set_scalar(&robot_location.try_into().unwrap(), EMPTY);
                        return Cow::Owned(next_coordinate);
                    }
                    '[' | ']' => {
                        walls_between.push(*next_tile);
                        next_coordinate = next_coordinate.shift(&direction).unwrap();
                        next_tile = warehouse
                            .get_scalar(&next_coordinate.try_into().unwrap())
                            .unwrap();
                    }
                    _ => panic!("Invalid tile {}", next_tile),
                }
            }
        }
        _ => panic!(),
    }
}

fn find_robot(warehouse: &Warehouse) -> Option<Coordinate> {
    for row in 0..warehouse.len() {
        for column in 0..warehouse[0].len() {
            if warehouse[row][column] == ROBOT {
                return Some(Coordinate {
                    x: column as isize,
                    y: row as isize,
                });
            }
        }
    }
    None
}

fn try_move<'a>(
    warehouse: &mut Warehouse,
    robot_location: &'a Coordinate,
    direction: &Direction,
) -> Cow<'a, Coordinate> {
    let next_space = get_next_empty_tile(robot_location, warehouse, &direction);
    match next_space {
        None => Cow::Borrowed(robot_location),
        Some(next) => {
            let adjacent = robot_location.shift(direction).unwrap();

            warehouse.set_scalar(&robot_location.try_into().unwrap(), EMPTY);

            if adjacent == next {
                warehouse.set_scalar(&next.try_into().unwrap(), ROBOT);
            } else {
                warehouse.set_scalar(&next.try_into().unwrap(), BOX);
                warehouse.set_scalar(&adjacent.try_into().unwrap(), ROBOT);
            }
            Cow::Owned(adjacent)
        }
    }
}

fn get_next_empty_tile(
    robot_location: &Coordinate,
    warehouse: &Warehouse,
    direction: &Direction,
) -> Option<Coordinate> {
    let mut next_location = robot_location.shift(direction).unwrap();
    loop {
        let next_tile = warehouse
            .get_scalar(&next_location.try_into().unwrap())
            .unwrap();
        match next_tile {
            &WALL => return None,
            &BOX => {
                next_location = next_location.shift(direction).unwrap();
            }
            &EMPTY => {
                return Some(next_location);
            }
            _ => unreachable!(),
        }
    }
}
fn count_score(warehouse: &Warehouse) -> usize {
    let mut score = 0;
    for row in 0..warehouse.len() {
        for column in 0..warehouse[0].len() {
            let tile = warehouse.get_scalar(&Index { x: column, y: row }).unwrap();
            if tile == &BOX {
                let gps_coordinate = 100 * row + column;
                score += gps_coordinate
            }
        }
    }
    score
}
fn count_wide_score(warehouse: &Warehouse) -> usize {
    let mut score = 0;
    for row in 0..warehouse.len() {
        for column in 0..warehouse[0].len() {
            let tile = warehouse.get_scalar(&Index { x: column, y: row }).unwrap();
            if tile == &'[' {
                let gps_coordinate = 100 * row + column;
                score += gps_coordinate
            }
        }
    }
    score

}
fn print_warehouse(warehouse: &Warehouse) {
    for row in 0..warehouse.len() {
        for column in 0..warehouse[0].len() {
            print!(
                "{}",
                warehouse.get_scalar(&Index { x: column, y: row }).unwrap()
            );
        }
        println!();
    }
}

fn resize_warehouse(warehouse: &mut Warehouse) {
    let row_n = warehouse.len();
    for row in 0..row_n {
        let new_row = Vec::with_capacity(row_n * 2);

        let old_row = mem::replace(&mut warehouse[row], new_row);
        let mut new_row = warehouse.get_mut(row).unwrap();

        old_row.into_iter().for_each(|ch| {
            if ch == 'O' {

                new_row.push('[');
                new_row.push(']');
            } else if ch == '@' {

                new_row.push('@');
                new_row.push('.');
            }
            else {

                new_row.push(ch);
                new_row.push(ch);
            }
        });
    }
}


#[cfg(test)]
mod tests {
    use crate::day_15::{count_score, count_wide_score, parse_input, print_warehouse, resize_warehouse, update_warehouse, update_wide_warehouse, Warehouse};
    use crate::graph::{Index, Matrix};

    const PREAMBLE: &str = include_str!("../resources/day_15/preamble.txt");
    const EASY: &str = include_str!("../resources/day_15/easy.txt");

    #[test]
    fn preamble() {
        let (mut warehouse, instructions) = parse_input(PREAMBLE);
        print_warehouse(&warehouse);
        println!();
        update_warehouse(&mut warehouse, &instructions);
        print_warehouse(&warehouse);
        println!();
        let left = count_score(&warehouse);
        let right = 10092;
        assert_eq!(left, right);
    }
    #[test]
    fn easy() {
        let (mut warehouse, instructions) = parse_input(EASY);
        update_warehouse(&mut warehouse, &instructions);
        let left = count_score(&warehouse);
        let right = 1563092;
        assert_eq!(left, right);
    }

    #[test]
    fn hard_preamble() {
        let (mut warehouse, instructions) = parse_input(PREAMBLE);
        resize_warehouse(&mut warehouse);
        println!("###START###");
        print_warehouse(&warehouse);
        update_wide_warehouse(&mut warehouse, &instructions);
        println!("###RESULT###");
        print_warehouse(&warehouse);
        let left = count_wide_score(&warehouse);
        let right = 9021;
        assert_eq!(left, right);
    }
    #[test]
    fn hard() {
        let (mut warehouse, instructions) = parse_input(EASY);
        resize_warehouse(&mut warehouse);
        println!("###START###");
        print_warehouse(&warehouse);
        update_wide_warehouse(&mut warehouse, &instructions);
        println!("###RESULT###");
        print_warehouse(&warehouse);
        let left = count_wide_score(&warehouse);
        let right = 1582688;
        assert_eq!(left, right);
    }
}
