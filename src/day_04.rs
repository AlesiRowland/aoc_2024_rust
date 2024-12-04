type WordSearch = Vec<Vec<char>>;

macro_rules! unwrap_or_return_false {
    ($op:expr) => {
        match $op {
            Some(value) => value,
            None => return false,
        }
    };
}

trait Matrix<T> {
    fn get_scalar(&self, point: &Point) -> Option<&T>;
}

impl Matrix<char> for WordSearch {
    fn get_scalar(&self, point: &Point) -> Option<&char> {
        self.get(point.y)?.get(point.x)
    }
}

struct MatrixPointsIterator {
    x_cursor: usize,
    y_cursor: usize,
    row_length: usize,
    column_length: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn shift(&self, direction: &Direction) -> Option<Point> {
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
            Direction::NorthEast => Some(Point {
                x: self.x.checked_add(1)?,
                y: self.y.checked_sub(1)?,
            }),
            Direction::NorthWest => Some(Point {
                x: self.x.checked_sub(1)?,
                y: self.y.checked_sub(1)?,
            }),
            Direction::SouthEast => Some(Point {
                x: self.x.checked_add(1)?,
                y: self.y.checked_add(1)?,
            }),
            Direction::SouthWest => Some(Point {
                x: self.x.checked_sub(1)?,
                y: self.y.checked_add(1)?,
            }),
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

fn iter_directions() -> [Direction; 8] {
    [
        Direction::North,
        Direction::East,
        Direction::West,
        Direction::South,
        Direction::NorthWest,
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
    ]
}

pub fn get_word_count(word: &str, word_search: &WordSearch) -> usize {
    let word_chars = word.chars().collect::<Vec<_>>();
    let mut count = 0;

    for y in 0..word_search.len() {
        for x in 0..word_search[0].len() {
            let start = Point { x, y };
            for directions in iter_directions() {
                if word_exists_in_direction(&start, &directions, &word_chars, word_search) {
                    count += 1
                }
            }
        }
    }
    count
}

fn word_exists_in_direction(
    start: &Point,
    direction: &Direction,
    word_chars: &[char],
    word_search: &WordSearch,
) -> bool {
    let mut next_index = Some(start.clone());

    for char in word_chars {
        let current = unwrap_or_return_false!(next_index.take());
        let ch = unwrap_or_return_false!(word_search.get_scalar(&current));

        if char != ch {
            return false;
        };
        next_index = current.shift(direction);
    }
    true
}

pub fn get_cross_word_count(word_search: &WordSearch) -> usize {
    let mut count = 0;

    for y in 0..word_search.len() {
        for x in 0..word_search[0].len() {
            let start = Point { x, y };
            count += xmas_found(&start, word_search) as usize;
        }
    }
    count
}

fn xmas_found(start: &Point, word_search: &WordSearch) -> bool {
    let origin = unwrap_or_return_false!(word_search.get_scalar(start));

    if origin != &'A' {
        return false;
    };

    // Check left Diagonal
    let upper_left_index = unwrap_or_return_false!(start.shift(&Direction::NorthWest));
    let upper_left = unwrap_or_return_false!(word_search.get_scalar(&upper_left_index));

    let lower_right_index = unwrap_or_return_false!(start.shift(&Direction::SouthEast));
    let lower_right = unwrap_or_return_false!(word_search.get_scalar(&lower_right_index));

    if !((upper_left == &'M' && lower_right == &'S') || (upper_left == &'S' && lower_right == &'M'))
    {
        return false;
    }

    let upper_right_index = unwrap_or_return_false!(start.shift(&Direction::NorthEast));
    let upper_right = unwrap_or_return_false!(word_search.get_scalar(&upper_right_index));
    let lower_left_index = unwrap_or_return_false!(start.shift(&Direction::SouthWest));
    let lower_left = unwrap_or_return_false!(word_search.get_scalar(&lower_left_index));

    if !((upper_right == &'M' && lower_left == &'S') || (upper_right == &'S' && lower_left == &'M'))
    {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::answers::{DAY_04_EASY, DAY_04_HARD};
    use crate::day_04::{get_cross_word_count, get_word_count, WordSearch};

    const INPUT: &str = include_str!("../resources/day_04/easy.txt");
    const PREAMBLE: &str = include_str!("../resources/day_04/preamble.txt");

    fn parse_input(input: &str) -> WordSearch {
        input.split('\n').map(|l| l.chars().collect()).collect()
    }

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
