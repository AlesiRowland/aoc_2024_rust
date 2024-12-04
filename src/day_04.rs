use std::collections::HashSet;
use std::fs::DirEntry;
use std::num::ParseIntError;

type WordSearch = Vec<Vec<char>>;

trait Matrix<T> {
    fn get_scalar(&self, point: &Point) -> Option<&T>;
}

impl Matrix<char> for WordSearch {
    fn get_scalar(&self, point: &Point) -> Option<&char> {
        self.get(point.y)?.get(point.x)
    }
}
fn parse_input(input: &str) -> WordSearch {
    input.split('\n').map(|l| l.chars().collect()).collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn move_in_direction(&self, direction: &Direction) -> Option<Point> {
        match direction {
            Direction::North => Some(Point {
                x: self.x,
                y: self.y.checked_sub(1)?,
            }),
            Direction::East => Some(Point {
                x: self.x.checked_add(1)?,
                y: self.y,
            }),
            Direction::South => Some(Point {
                x: self.x,
                y: self.y.checked_add(1)?,
            }),
            Direction::West => Some(Point {
                x: self.x.checked_sub(1)?,
                y: self.y,
            }),
        }
    }
    fn move_in_directions(&self, directions: &[Direction]) -> Option<Point> {
        directions.iter().fold(Some(self.clone()), |a, b| {
            a.and_then(|c| c.move_in_direction(b))
        })
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn iter_directions() -> [Vec<Direction>; 8] {
    [
        vec![Direction::North],
        vec![Direction::East],
        vec![Direction::South],
        vec![Direction::West],
        vec![Direction::North, Direction::East],
        vec![Direction::North, Direction::West],
        vec![Direction::South, Direction::East],
        vec![Direction::South, Direction::West],
    ]
}

fn get_word_count(word: &str, word_search: &WordSearch) -> usize {
    let word_chars = word.chars().collect::<Vec<_>>();
    let mut count = 0;

    for y in (0..word_search.len()) {
        for x in (0..word_search[0].len()) {
            let start = Point { x, y };
            for directions in iter_directions() {
                if word_exists_in_direction(&start, &directions, &word_chars, &word_search) {
                    count += 1
                }
            }
        }
    }
    count
}

fn word_exists_in_direction(
    start: &Point,
    directions: &Vec<Direction>,
    word_chars: &Vec<char>,
    word_search: &WordSearch,
) -> bool {
    let mut next_index = Some(start.clone());

    for char in word_chars {
        let Some(current) = next_index.take() else {
            return false;
        };

        let Some(ch) = word_search.get_scalar(&current) else {
            return false;
        };

        if char != ch {
            return false;
        };
        next_index = current.move_in_directions(&directions);
    }
    true
}

fn get_cross_word_count(word_search: &WordSearch) -> usize {
    let mut count = 0;

    for y in (0..word_search.len()) {
        for x in (0..word_search[0].len()) {
            let start = Point { x, y };
            if xmas_found(&start, &word_search) {
                count += 1
            }

        }
    }

    count
}

fn xmas_found(start: &Point, word_search: &WordSearch) -> bool {
    let Some(line) = word_search.get(start.y) else {
        return false
    };
    let Some(origin) = line.get(start.x) else {
        return false
    };

    if origin != &'A' {
        return false
    };

    // Check left Diagonal
    let Some(upper_left_index) = start.move_in_directions(&[Direction::North, Direction::West])
    else {
        return false
    };
    let Some(lower_right_index) = start.move_in_directions(&[Direction::South, Direction::East])
    else {
        return false
    };

    let Some(upper_left) = word_search.get_scalar(&upper_left_index) else {
        return false
    };
    let Some(lower_right) = word_search.get_scalar(&lower_right_index) else {
        return false
    };

    if !((upper_left == &'M' && lower_right == &'S') || (upper_left == &'S' && lower_right == &'M'))
    {
        return false
    }

    let Some(upper_right_index) = start.move_in_directions(&[Direction::North, Direction::East])
    else {
        return false
    };
    let Some(lower_left_index) = start.move_in_directions(&[Direction::South, Direction::West])
    else {
        return false
    };

    let Some(upper_right) = word_search.get_scalar(&upper_right_index) else {
        return false
    };
    let Some(lower_left) = word_search.get_scalar(&lower_left_index) else {
        return false
    };

    if !((upper_right == &'M' && lower_left == &'S') || (upper_right == &'S' && lower_left == &'M'))
    {
        return false
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::answers::{DAY_04_EASY, DAY_04_HARD};
    use crate::day_04::{get_cross_word_count, get_word_count, parse_input};

    const INPUT: &str = include_str!("../resources/day_04/easy.txt");
    const PREAMBLE: &str = include_str!("../resources/day_04/preamble.txt");
    #[test]
    fn preamble() {
        let word_search = parse_input(PREAMBLE);
        let left = get_word_count("XMAS", &word_search);
        let right = 18;
        assert_eq!(left, right);
    }
    #[test]
    fn easy() {
        let word_search = parse_input(INPUT);
        let left = get_word_count("XMAS", &word_search);
        let right = DAY_04_EASY;
        assert_eq!(left, right);
    }

    #[test]
    fn hard() {
        let word_search = parse_input(INPUT);
        let left = get_cross_word_count(&word_search);
        let right = DAY_04_HARD;
        assert_eq!(left, right);
    }
}
