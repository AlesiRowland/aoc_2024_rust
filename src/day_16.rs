use crate::graph::{Coordinate, Direction, Matrix};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::os::raw::c_float;

// Reindeer maze
// BFS -> keep searching the maze until a

struct Reindeer {
    location: Coordinate,
    direction: Direction,
}

fn get_shortest_path_score(maze: &Vec<Vec<char>>) -> Option<usize> {
    let start = find_start(maze)?;
    let reindeer = Reindeer {
        location: start,
        direction: Direction::East,
    };
    let mut finishers = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_front((0usize, reindeer));

    let mut visited = HashMap::new();

    while let Some((score, Reindeer {
        location,
        direction,
    })) = queue.pop_back()
    {


        match visited.get(&location) {
            Some(best_score) if *best_score <= score => {
                continue
            },

            _ => {
               visited.insert(location, score);
            }
        }

        // try current direction.
        let next = location.shift(&direction).unwrap();
        let next_tile = maze.get_scalar(&next.try_into().unwrap()).unwrap();

        if next_tile == &'E' {

           finishers.push((score + 1));
        } else if next_tile == &'.' {
            queue.push_front((score + 1 , Reindeer {location: next, direction}));
        }

        // try the other two directions

        let directions = [direction.rotate_90_degrees_clockwise(), direction.rotate_90_degrees_counter_clockwise()];
        directions.into_iter().for_each(|direction| {
            // try current direction.
            let next = location.shift(&direction).unwrap();
            let next_tile = maze.get_scalar(&next.try_into().unwrap()).unwrap();

            if next_tile == &'E' {
                finishers.push((score + 1001));
            } else if next_tile == &'.' {
                queue.push_front((score + 1001 , Reindeer {location: next, direction}));
            }

        })
    };
    finishers.into_iter().min()
}


// struct Path {
//     current_score: usize,
//     locations: Vec<Coordinate>,
//     direction :Direction,
// }
//
//
// impl Path {
//     pub fn new(start: Coordinate, direction: Direction) -> Self {
//         let locations = vec![start];
//         let current_score = 0;
//         Path {
//             current_score, locations, direction
//         }
//     }
//
//     fn current_location(&self) -> &Coordinate {
//        self.locations.last().unwrap()
//     }
// }

// fn find_best_spots(maze: &Vec<Vec<char>>) -> Option<HashSet<Coordinate>> {
//     // you need to do the same as before, except now store the visited as part of the
//     // so for one path, you need to know
//     // the score of the path
//     // the route taken
//     let start = find_start(maze)?;
//     // let mut queue = VecDeque::new();
//     let mut best_spots = HashSet::new();
//     let mut path = Path::new(start, Direction::East);
//     let mut queue = VecDeque::new();
//     queue.push_front(path);
//
//     let mut visited: HashMap<Coordinate, Path> = HashMap::new();
//     while let Some(path)= queue.pop_back() {
//         match visited.get(path.current_location()) {
//             Some(old_path) if old_path.current_score <= path.current_score => {
//                 continue
//             },
//             _ => {
//                 visited.insert(*path.current_location(), path);
//             }
//         }
//
//         // try current direction.
//         let next = path.current_location().shift(&path.direction).unwrap();
//         let next_tile = maze.get_scalar(&next.try_into().unwrap()).unwrap();
//
//         // if next_tile == &'E' {
//         //
//         //     finishers.push((score + 1));
//         // } else if next_tile == &'.' {
//         //     queue.push_front((score + 1 , Reindeer {location: next, direction}));
//         // }
//
//
//
//
//     };
//     Some(best_spots)


// }
fn find_start(maze: &Vec<Vec<char>>) -> Option<Coordinate> {
    for row in 0..maze.len() {
        for column in 0..maze[0].len() {
            let tile = maze[row][column];
            if tile == 'S' {
                return Some(Coordinate {
                    x: column as isize,
                    y: row as isize,
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::answers::DAY_16_EASY;
    use crate::day_16::get_shortest_path_score;
    const PREAMBLE: &str = include_str!("../resources/day_16/preamble.txt");
    const EASY: &str = include_str!("../resources/day_16/easy.txt");

    fn parse_input(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|l| l.chars().collect()).collect()
    }

    #[test]
    fn preamble() {
        let input = parse_input(PREAMBLE);
        let left = get_shortest_path_score(&input);
        let right = Some(7036);
        assert_eq!(left, right);
    }
    #[test]
    fn easy() {
        let input = parse_input(EASY);
        let left = get_shortest_path_score(&input);
        let right = Some(DAY_16_EASY);
        assert_eq!(left, right)
    }
}
