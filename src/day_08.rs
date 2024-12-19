use std::collections::{HashMap, HashSet};
use std::num::{TryFromIntError};

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
struct Point<T: Copy> {
    x: T,
    y: T,
}

pub(crate) trait Matrix<T> {
    fn get_scalar(&self, index: &Index) -> Option<&T>;
}

impl<T> Matrix<T> for Vec<Vec<T>> {
    fn get_scalar(&self, index: &Index) -> Option<&T> {
        self.get(index.y).and_then(|row| row.get(index.x))
    }
}

type Index = Point<usize>;

impl TryFrom<Point<isize>> for Index {
    type Error = TryFromIntError;
    fn try_from(value: Point<isize>) -> Result<Self, Self::Error> {
        Ok(Self { y: value.y.try_into()?, x: value.x.try_into()? })
    }
}

impl TryFrom<Index> for Point<isize> {
    type Error = TryFromIntError;
    fn try_from(value: Index) -> Result<Self, Self::Error> {
        Ok(Self { y: value.y.try_into()?, x: value.x.try_into()? })
    }
}


fn find_all_anti_nodes(matrix: &Vec<Vec<char>>) -> HashSet<Index> {
    let mut anti_nodes = HashSet::new();
    let antenna_positions = find_antenna_positions(matrix);
    for antennas in antenna_positions.values() {
        anti_nodes.extend(get_anti_nodes(antennas, matrix));
    }
    anti_nodes
}
fn get_anti_nodes<M: Matrix<char>>(antennas: &Vec<Point<isize>>, matrix: &M) -> Vec<Index> {
    let mut anti_nodes = Vec::new();
    let mut i = 1;
    for first in &antennas[0..antennas.len() - 1] {
        for second in &antennas[i..] {


            let anti_node = get_anti_node(first, second);
            if let Some(anti_node) = anti_node {
               if let Ok(anti_node) = anti_node.try_into() {
                   if let Some(_) = matrix.get_scalar(&anti_node) {

                       anti_nodes.push(anti_node);
                   }
               }
            }
            let anti_node = get_anti_node(second, first);
            if let Some(anti_node) = anti_node {
                if let Ok(anti_node) = anti_node.try_into() {
                    if let Some(_) = matrix.get_scalar(&anti_node) {

                        anti_nodes.push(anti_node);
                    }
                }
            }
        }
        i += 1
    }
    anti_nodes
}

fn get_anti_node(first: &Point<isize>, second: &Point<isize>) -> Option<Point<isize>> {
    let diff_x = first.x.checked_sub(second.x).unwrap();
    let diff_y = first.y.checked_sub(second.y).unwrap();

    let x = first.x.checked_add(diff_x).unwrap();
    let y = first.y.checked_add(diff_y).unwrap();
    Some(Point { x, y })
}

fn find_antinodes_hard(matrix: &Vec<Vec<char>>) -> Vec<Point<isize>> {
    let mut anti_nodes = Vec::new();
    let antenna_positions = find_antenna_positions(matrix);
    for antennas in antenna_positions.values() {
        let mut i = 1;
        for first in &antennas[0..antennas.len() - 1] {
            for second in &antennas[i..] {
                let nodes = get_all_anti_nodes(&[*first, *second], matrix);
                anti_nodes.extend(nodes);
                let nodes = get_all_anti_nodes(&[*second, *first], matrix);
                anti_nodes.extend(nodes)
            }
            i += 1
        }
    }

    anti_nodes
}
fn get_all_anti_nodes(segment: &[Point<isize>; 2], matrix: &Vec<Vec<char>> ) -> Vec<Point<isize>> {
    let (first, second) = (segment[0], segment[1]);
    let diff_x = first.x.checked_sub(second.x).unwrap();
    let diff_y = first.y.checked_sub(second.y).unwrap();

    let mut anti_nodes = Vec::new();

    let mut anti_node = first.clone();
    loop {
        let Ok(index)= anti_node.try_into() else {
            break
        };

        if let None = matrix.get_scalar(&index) {
            break
        }
        anti_nodes.push(anti_node);

        let x = anti_node.x.checked_add(diff_x).unwrap();
        let y = anti_node.y.checked_add(diff_y).unwrap();
        anti_node = Point {x, y}
    }
    anti_nodes
}

fn find_antenna_positions(matrix: &Vec<Vec<char>>) -> HashMap<char, Vec<Point<isize>>> {
    let mut antennas = HashMap::new();
    for row in 0..matrix.len() {
        for column in 0..matrix[0].len() {
            let index = Index { x: column, y: row };
            let ch = matrix.get_scalar(&index).unwrap();
            if ch != &'.' {
                let mut common_antennas = antennas.entry(*ch).or_insert_with(Vec::new);
                common_antennas.push(index.try_into().unwrap());
            }
        }
    }
    antennas
}

#[cfg(test)]
mod tests {
    use std::arch::aarch64::vreinterpret_f32_f64;
    use std::collections::{HashMap, HashSet};
    use crate::answers::{DAY_08_EASY, DAY_08_HARD};
    use crate::day_08::{find_all_anti_nodes, find_antinodes_hard, Index, Matrix};

    const PREAMBLE: &str = include_str!("../resources/day_08/preamble.txt");
    const EASY: &str = include_str!("../resources/day_08/easy.txt");

    fn parse_input(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|l| l.chars().collect()).collect()
    }

    #[test]
    fn preamble() {
        let input = parse_input(PREAMBLE);
        let left = find_all_anti_nodes(&input);
        for row in 0..input.len() {
            for column in 0..input[row].len() {
                let ind = Index { x: column, y: row };
                if left.contains(&ind) {
                    print!("x");
                } else {
                    print!("{}", input.get_scalar(&ind).unwrap())
                }
            }

            print!("\n");
        }
        let right = 14;
        assert_eq!(left.len(), right);
    }
    #[test]
    fn easy() {
        let left = find_all_anti_nodes(&parse_input(EASY)).len();
        let right = DAY_08_EASY;
        assert_eq!(left, right);
    }
    #[test]
    fn hard() {

        let left = find_antinodes_hard(&parse_input(EASY)).iter().collect::<HashSet<_>>().len();
        let right = DAY_08_HARD;
        assert_eq!(left, right);
    }
}
