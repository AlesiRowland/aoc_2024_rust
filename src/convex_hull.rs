use crate::graph::{Coordinate, Direction, Point};
use std::cmp::{Ordering, PartialEq};
use std::collections::{HashSet, VecDeque};

enum Orientation {
    Clockwise,
    AntiClockwise,
    Collinear,
}

impl From<isize> for Orientation {
    fn from(value: isize) -> Self {
        if value > 0 {
            Orientation::Clockwise
        } else if value < 0 {
            Orientation::AntiClockwise
        } else {
            Orientation::Collinear
        }
    }
}
pub fn find_convex_hull(polygon: &HashSet<Coordinate>) -> Vec<&Coordinate> {
    let bottom_left = get_bottom_left(polygon).unwrap();
    let mut polygon = polygon.iter().collect::<Vec<_>>();
    polygon.sort_by(|&p1, &p2| compare_by_polar_angle(&bottom_left, p1, p2));
    let mut polygon_reversed = polygon.into_iter();
    let p0 = polygon_reversed.next().unwrap();
    let p1 = polygon_reversed.next().unwrap();

    let mut boundary = Vec::new();
    boundary.push(p0);
    boundary.push(p1);

    for p2 in polygon_reversed {
        loop {
            let p0 = boundary.get(boundary.len() - 2).unwrap();
            let p1 = boundary.last().unwrap();

            match orientation(p0, p1, p2).unwrap() {
                Orientation::Clockwise | Orientation::Collinear => {
                    boundary.pop().unwrap();
                }
                Orientation::AntiClockwise => {
                    boundary.push(p2);
                    break;
                }
            }
        }
    }
    boundary
}




fn sort_by_polar_angle(points: &mut Vec<Coordinate>, p0: &Coordinate) {
    points.sort_by(|p1, p2| compare_by_polar_angle(p0, p1, p2))
}

fn gradient_of(p0: &Coordinate, p1: &Coordinate) -> f64 {
    -(p1.y as f64 - p0.y as f64) / (p1.x as f64 - p0.x as f64)
}
fn compare_by_polar_angle(p0: &Coordinate, p1: &Coordinate, p2: &Coordinate) -> Ordering {

    match orientation(p0, p1, p2).unwrap() {
        Orientation::Clockwise => Ordering::Greater,
        Orientation::AntiClockwise => Ordering::Less,
        Orientation::Collinear => {
            let p1_dist_squared = distance_squared(p0, p1);
            let p2_dist_squared = distance_squared(p0, p2);
            if p1_dist_squared > p2_dist_squared {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }
    // let gradient01 = gradient_of(p0, p1);
    // let gradient02 = gradient_of(p0, p2);
    //
    // if gradient01 < gradient02 {
    //     Ordering::Less
    // } else if gradient01 > gradient02 {
    //     Ordering::Greater
    // } else {
    //     let p1_dist_squared = distance_squared(p0, p1);
    //     let p2_dist_squared = distance_squared(p0, p2);
    //     if p1_dist_squared > p2_dist_squared {
    //         Ordering::Greater
    //     } else {
    //         Ordering::Less
    //     }
    // }
}

fn orientation(p1: &Coordinate, p2: &Coordinate, p3: &Coordinate) -> Option<Orientation> {
    let y12 = p2.y.checked_sub(p1.y)?;
    let x23 = p3.x.checked_sub(p2.x)?;
    let y23 = p3.y.checked_sub(p2.y)?;
    let x12 = p2.x.checked_sub(p1.x)?;
    Some(Orientation::from(-(y12 * x23 - y23 * x12)))
}

fn distance_squared(p1: &Coordinate, p2: &Coordinate) -> Option<isize> {
    let x_diff = p1.x.checked_sub(p2.x)?;
    let x_diff_squared = x_diff.checked_pow(2)?;
    let y_diff = p1.y.checked_sub(p2.y)?;
    let y_diff_squared = y_diff.checked_pow(2)?;
    x_diff_squared.checked_add(y_diff_squared)
}

pub fn get_bottom_left(polygon: &HashSet<Coordinate>) -> Option<&Coordinate> {
    let bottom = get_coordinates_at_bottom(polygon)?;
    let bottom_left = bottom.iter().min_by(|&p1, &p2| p1.x.cmp(&p2.x))?;
    Some(bottom_left)
}

fn get_coordinates_at_bottom(polygon: &HashSet<Coordinate>) -> Option<Vec<&Coordinate>> {
    let max_y = polygon.iter().max_by(|p1, p2| p1.y.cmp(&p2.y))?;
    Some(polygon.iter().filter(|&p| p.y == max_y.y).collect())
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::f64::INFINITY;
    use crate::convex_hull::{compare_by_polar_angle, gradient_of, orientation, Orientation};
    use crate::graph::Coordinate;
    #[test]
    fn test_gets_infinity() {
        let p0 = Coordinate {x: 0, y: 10};
        let p1 = Coordinate  {x: 0, y: 6};
        let left = gradient_of(&p0, &p1);
        let right = f64::INFINITY;
        assert_eq!(left, right)
    }

    #[test]
    fn test_gets_neg_infinity() {
        let p0 = Coordinate {x: 0, y: 6};
        let p1 = Coordinate  {x: 0, y: 10};
        let left = gradient_of(&p0, &p1);
        let right = f64::NEG_INFINITY;
        assert_eq!(left, right)
    }

    #[test]
    fn test_gets_gradient() {
        let p0 = Coordinate {x: 1, y: 6};
        let p1 = Coordinate {x: 2, y: 5};
        let left = gradient_of(&p0, &p1);
        let right = 1.;
        assert_eq!(left, right)
    }

    #[test]
    fn test_gets_less_than_ordering() {
        let p0 = Coordinate {x: 1, y: 6};
        let p1 = Coordinate {x: 2, y: 3};
        let p2 = Coordinate {x: 3, y: 4};
        let left = compare_by_polar_angle(&p0, &p1, &p2);
        let right = Ordering::Greater;
        assert_eq!(left, right);
    }
    #[test]
    fn test_gets_greater_than_ordering() {
        let p0 = Coordinate {x: 1, y: 6};
        let p1 = Coordinate {x: 2, y: 4};
        let p2 = Coordinate {x: 3, y: 0};
        let left = compare_by_polar_angle(&p0, &p1, &p2);
        let right = Ordering::Less;
        assert_eq!(left, right);
    }

    #[test]
    fn collinear_less_than() {
        let p0 = Coordinate {x: 1, y: 2};
        let p1 = Coordinate {x: 2, y: 3};
        let p2 = Coordinate {x: 3, y: 4};
        let left = compare_by_polar_angle(&p0, &p1, &p2);
        let right = Ordering::Less;
        assert_eq!(left, right);
    }

    #[test]
    fn collinear_greater_than() {
        let p0 = Coordinate { x: 1, y: 2 };
        let p1 = Coordinate { x: 3, y: 2 };
        let p2 = Coordinate { x: 2, y: 2 };
        let left = compare_by_polar_angle(&p0, &p1, &p2);
        let right = Ordering::Greater;
        assert_eq!(left, right);
    }

    #[test]
    fn real_case() {

        let p0 = Coordinate { x: 2, y: 3 };
        let p1 = Coordinate { x: 2, y: 2 };
        let p2 = Coordinate { x: 3, y: 2 };
        let left = compare_by_polar_angle(&p0, &p1, &p2);
        let right = Ordering::Greater;
        assert_eq!(left, right);
    }


    #[test]
    fn test_ordering() {
        let p0 = Coordinate { x: 2, y: 3 };
        let p1 = Coordinate { x: 2, y: 2 };
        let p2 = Coordinate { x: 3, y: 2 };
        let mut coordinates = [p1, p0, p2];
        coordinates.sort_by(|p1, p2| compare_by_polar_angle(&p0, p1, p2));
        let right = [p0, p2, p1];
        assert_eq!(coordinates, right)
    }


    #[test]
    fn test_resl() {
        let p0 = Coordinate { x: 2, y: 3 };
        let p1 = Coordinate { x: 2, y: 2 };
        let p2 = Coordinate { x: 3, y: 2 };

        let case1 = compare_by_polar_angle(&p0, &p1, &p2);
        let case2 = compare_by_polar_angle(&p0, &p2, &p1);
        let case3 = compare_by_polar_angle(&p0, &p0, &p1);
        let case4 = compare_by_polar_angle(&p0, &p0, &p2);
        let case5 = compare_by_polar_angle(&p0, &p1, &p0);
        let case6 = compare_by_polar_angle(&p0, &p2, &p0);


        println!("{:?}", case1);
        println!("{:?}", case2);
    }

}