use crate::convex_hull::{find_convex_hull, get_bottom_left};
use crate::graph::{Coordinate, Direction, Matrix, Point};
use std::collections::{BTreeSet, HashMap, HashSet};
use clap::Parser;

fn iter_directions() -> [Direction; 4] {
    [
        Direction::North,
        Direction::East,
        Direction::West,
        Direction::South,
    ]
}

fn get_price(land: &Vec<Vec<char>>) -> usize {
    let mut visited = HashSet::new();
    let mut price = 0;
    for row in 0..land.len() {
        for column in 0..land[0].len() {
            let current = Point { x: column, y: row }.try_into().unwrap();

            if visited.contains(&current) {
                continue;
            }

            let region = find_region2(&current, land);
            match region {
                None => continue,
                Some(Region {
                    coordinates,
                    area,
                    perimeter,
                    ..
                }) => {
                    visited.extend(coordinates);
                    price += area * perimeter;
                }
            }
        }
    }
    price
}

fn get_discounted_price(land: &Vec<Vec<char>>) -> usize {
    let mut visited = HashSet::new();
    let mut price = 0;
    for row in 0..land.len() {
        for column in 0..land[0].len() {
            let current = Point { x: column, y: row }.try_into().unwrap();

            if visited.contains(&current) {
                continue;
            }

            let region = find_region2(&current, land);
            match region {
                None => continue,
                Some(Region {
                         coordinates,
                         area,
                         n_sides,
                         ..
                     }) => {
                    visited.extend(coordinates);
                    price += area * n_sides;
                }
            }
        }
    }
    price
}
fn get_regions(land: &Vec<Vec<char>>) -> Vec<Region> {
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    for row in 0..land.len() {
        for column in 0..land[0].len() {
            let current = Point { x: column, y: row }.try_into().unwrap();
            if visited.contains(&current) {
                continue;
            }
            let region = find_region(&current, land);
            match region {
                None => continue,
                Some(region) => {
                    visited.extend(region.coordinates.clone());
                    regions.push(region);
                }
            }
        }
    }
    regions
}
struct Region {
    tile: char,
    coordinates: HashSet<Point<isize>>,
    area: usize,
    perimeter: usize,
    n_sides: usize,
}

fn find_region(start: &Coordinate, land: &Vec<Vec<char>>) -> Option<Region> {
    // Walk and flood fill, adding 1 whenever a side does not connect and adding 1 whenever we find a value

    let mut area = 0;
    let mut perimeter = 0;

    let mut stack = Vec::new();
    let mut coordinates = HashSet::new();

    let first = land.get_scalar(&start.try_into().unwrap())?;
    stack.push(start.clone());

    while let Some(current) = stack.pop() {
        area += 1;
        coordinates.insert(current.clone());

        for direction in iter_directions() {

            let Some(next) = current.shift(&direction) else {
                perimeter += 1;
                continue;
            };

            let index = match next.try_into() {
                Ok(index) => index,
                Err(_) => {
                    perimeter += 1;
                    continue;
                }
            };

            if coordinates.contains(&next) || stack.contains(&next) {
                continue;
            }

            let Some(tile) = land.get_scalar(&index) else {
                perimeter += 1;
                continue;
            };

            if tile == first {
                stack.push(next);
            } else {
                perimeter += 1;
            }
        }
    }
    Some(Region {
        tile: *first,
        coordinates,
        area,
        perimeter,
        n_sides: 0
    })
}


fn find_region2(start: &Coordinate, land: &Vec<Vec<char>>) -> Option<Region> {
    // Walk and flood fill, adding 1 whenever a side does not connect and adding 1 whenever we find a value

    let mut area = 0;
    let mut perimeter = Vec::new();

    let mut stack = Vec::new();
    let mut coordinates = HashSet::new();

    let first = land.get_scalar(&start.try_into().unwrap())?;
    stack.push(start.clone());

    while let Some(current) = stack.pop() {
        area += 1;
        coordinates.insert(current.clone());

        for direction in iter_directions() {

            let Some(next) = current.shift(&direction) else {
                let side = (current, direction);
                perimeter.push(side);
                continue;
            };

            let index = match next.try_into() {
                Ok(index) => index,
                Err(_) => {
                    let side = (current, direction);
                    perimeter.push(side);
                    continue;
                }
            };

            if coordinates.contains(&next) || stack.contains(&next) {
                continue;
            }

            let Some(tile) = land.get_scalar(&index) else {
                let side = (current, direction);
                perimeter.push(side);
                continue;
            };

            if tile == first {
                stack.push(next);
            } else {
                let side = (current, direction);
                perimeter.push(side);
            }
        }
    }

    let perimeter_length = perimeter.len();
    // convert to a hash and start popping

    let mut n_sides = 0;
    let mut perimeter = perimeter.into_iter().collect::<HashSet<_>>();

    loop {
        // horrible but rust sucks at this part
        let Some(next) = perimeter.iter().next() else {
            break
        };
        let next = next.clone();
        let (current, direction)= perimeter.take(&next).unwrap();

        match direction {
            direction @ (Direction::North | Direction::South)=> {
                let mut left = (current.shift_west().unwrap(), direction.clone());
                while perimeter.remove(&left) {
                    left = (left.0.shift_west().unwrap(), left.1);
                }
                let mut right = (current.shift_east().unwrap(), direction.clone());
                while perimeter.remove(&right) {
                    right = (right.0.shift_east().unwrap(), right.1);
                }
                n_sides += 1;
            },
            direction @(Direction::West| Direction::East) => {
                let mut top = (current.shift_north().unwrap(), direction.clone());
                while perimeter.remove(&top) {
                    top = (top.0.shift_north().unwrap(), top.1);
                }
                let mut bottom = (current.shift_south().unwrap(), direction.clone());
                while perimeter.remove(&bottom) {
                    bottom = (bottom.0.shift_south().unwrap(), bottom.1);
                }
                n_sides += 1;
            }
            _ => unreachable!()
        }
    }

    // how do i get the adjacent
    Some(Region {
        tile: *first,
        coordinates,
        area,
        perimeter: perimeter_length,
        n_sides:n_sides
    })
}

fn find_neighbours(
    current: &Point<isize>,
    coordinates: &HashSet<Point<isize>>,
) -> HashMap<Direction, Point<isize>> {
    let mut neighbours = HashMap::new();
    for direction in iter_directions() {
        if let Some(next) = current.shift(&direction) {
            if coordinates.contains(&next) {
                neighbours.insert(direction, next);
            }
        }
    }
    neighbours
}



#[cfg(test)]
mod tests {
    use crate::day_12::{get_discounted_price, get_price};

    const PREAMBLE: &str = include_str!("../resources/day_12/preamble.txt");
    const INPUT: &str = include_str!("../resources/day_12/input.txt");

    fn parse_input(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }

    #[test]
    fn preamble() {
        let input = parse_input(PREAMBLE);
        let left = get_price(&input);
        let right = 1930;
        assert_eq!(left, right)
    }
    #[test]
    fn easy() {
        let input = parse_input(INPUT);
        let left = get_price(&input);
        let right = 1477762;
        assert_eq!(left, right)
    }
    #[test]
    fn preamble_hard() {
        let input = parse_input(PREAMBLE);
        let left = get_discounted_price(&input);
        let right =1206;
        assert_eq!(left, right)
    }
    #[test]
    fn hard() {
        let input = parse_input(INPUT);
        let left = get_discounted_price(&input);
        let right =923480;
        assert_eq!(left, right)
    }
}
