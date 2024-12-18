use std::os::unix::raw::off_t;
use crate::graph::Coordinate;
use regex::Regex;


#[derive(Debug)]
struct Velocity {
    x: isize,
    y : isize,
}

struct Dimensions {
    width: isize,
    height: isize,
}

#[derive(Debug)]
struct Robot {
    point: Coordinate,
    velocity: Velocity,
}

impl Robot {
    fn move_for(&mut self, seconds: isize, dimensions: &Dimensions) {
        self.point.x = (self.point.x +  self.velocity.x * seconds).rem_euclid(dimensions.width);
        self.point.y = (self.point.y + self.velocity.y * seconds).rem_euclid(dimensions.height);
        if self.point.x >= dimensions.width || self.point.y >= dimensions.height {
            panic!()
        }
    }

}

fn find_quadrant_score(robots: &Vec<Robot>, dimensions: &Dimensions) -> isize {
    let mut quadrant_scores = [0, 0, 0, 0];
    let x_lim = dimensions.width / 2;
    let y_lim = dimensions.height / 2;
    print_robots(robots, dimensions);
    robots
        .iter()
        .for_each(|robot| {
            if robot.point.x < x_lim {
                if robot.point.y < y_lim {
                   quadrant_scores[0] += 1;
                } else if robot.point.y > y_lim {
                    quadrant_scores[3] += 1;
                }
            }  else if robot.point.x > x_lim {
                // is top
                if robot.point.y < y_lim {
                    quadrant_scores[1] += 1;
                } else if robot.point.y > y_lim {
                    quadrant_scores[2] += 1;
                }
            }
        });
    quadrant_scores.iter().product()
}

fn check_safety(robots: &mut Vec<Robot>, seconds: isize, dimensions: &Dimensions) -> isize {
    robots.iter_mut().for_each(|robot| robot.move_for(seconds, dimensions));
    find_quadrant_score(robots, dimensions)
}
fn print_robots(robots: &Vec<Robot>, dimensions: &Dimensions) {

    for row in 0..dimensions.height {
        for column in 0..dimensions.width {
            let coordinate=  Coordinate {x: column, y: row};
            let ch = robots.iter().filter(|robot| robot.point == coordinate).count();
            if ch == 0 {
                print!(".")
            } else {
                print!("{}", ch)
            }

        }
        println!()
    }

    println!()
}
#[cfg(test)]
mod tests {
    use crate::day_14::{check_safety, Dimensions, Robot, Velocity};
    use crate::graph::Coordinate;
    use lazy_static::lazy_static;
    use regex::Regex;

    const PREAMBLE: &str = include_str!("../resources/day_14/preamble.txt");
    const INPUT: &str = include_str!("../resources/day_14/input.txt");

    fn parse_input(input: &str) -> Vec<Robot> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^p=(?<x>-?\d+),(?<y>-?\d+) v=(?<x_inc>-?\d+),(?<y_inc>-?\d+)$").unwrap();
        };
        input
            .lines()
            .map(|line| {
                let captures = RE.captures(line).unwrap();
                let x = captures.name("x").unwrap().as_str().parse().unwrap();
                let y = captures.name("y").unwrap().as_str().parse().unwrap();
                let x_inc = captures.name("x_inc").unwrap().as_str().parse().unwrap();
                let y_inc = captures.name("y_inc").unwrap().as_str().parse().unwrap();
                Robot {
                    point: Coordinate { x, y },
                    velocity: Velocity{x: x_inc, y: y_inc},
                }
            })
            .collect()
    }
    #[test]
    fn preamble() {
        let dimensions: Dimensions = Dimensions {
            width: 11,
            height: 7,
        };
        let mut input = parse_input(PREAMBLE);
        println!("{:?}", input.len());
        let left = check_safety(&mut input, 100, &dimensions);
        let right = 12;
        assert_eq!(left, right)
    }

    #[test]
    fn easy() {
        let dimensions: Dimensions = Dimensions {
            width: 101,
            height: 103,
        };
        let mut input = parse_input(INPUT);
        let left = check_safety(&mut input, 100, &dimensions);
        let right = 211773366;
        assert_eq!(left, right)
        // let right = 0;
    }
}
