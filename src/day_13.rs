use crate::graph::{Coordinate, Point};
struct Button {
    x: f64,
    y: f64,
}

type Prize = Point<f64>;


struct Trial {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

// Apparently this is simultaneous equations
/*
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400


*/
fn get_cost(trial: &Trial) -> isize{
    let Prize {x: px, y: py} = trial.prize;
    let Button {x: ax, y: ay} = trial.button_a;
    let Button {x: bx, y: by} = trial.button_b;

    let an = (px*by - py*bx) / (ax*by - ay*bx);
    let bn = (px-ax * an) / bx;

    if (an.fract() > 0. || bn.fract() > 0.) {
       return 0
    }
    let an = an as isize;
    let bn = bn as isize;
    an * 3 + bn
}

fn fewest_tokens(trials: &Vec<Trial>) -> isize{
    trials.iter().map(get_cost).sum()
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use lazy_static::lazy_static;

    use crate::day_13::{fewest_tokens, Button, Prize, Trial};
    const PREAMBLE: &str = include_str!("../resources/day_13/preamble.txt");
    const EASY: &str = include_str!("../resources/day_13/easy.txt");

    fn parse_input(input: &str) -> Vec<Trial> {

        input.split("\n\n").map(|trial| {
            let mut lines = trial.lines();
            let button_a = parse_button_line(lines.next().unwrap());
            let button_b = parse_button_line(lines.next().unwrap());
            let prize = parse_prize_line(lines.next().unwrap());
            Trial {
                button_a,
                button_b ,prize
            }
        }).collect()
    }

    fn parse_hard_input(input: &str) -> Vec<Trial> {

        input.split("\n\n").map(|trial| {
            let mut lines = trial.lines();
            let button_a = parse_button_line(lines.next().unwrap());
            let button_b = parse_button_line(lines.next().unwrap());
            let mut prize = parse_prize_line(lines.next().unwrap());
            prize.x += 1_000_000_000_000_0.;
            prize.y += 1_000_000_000_000_0.;

            Trial {
                button_a,
                button_b ,prize
            }
        }).collect()
    }
    fn parse_button_line(button_line: &str) -> Button {
        lazy_static! {
            static ref BUTTON: regex::Regex = Regex::new(r"^Button (A|B): X\+(?<x>\d+), Y\+(?<y>\d+)$").unwrap();
        };



        let captures = BUTTON.captures(button_line).unwrap();
        let x_increment = captures.name("x").unwrap().as_str().parse().unwrap();
        let y_increment = captures.name("y").unwrap().as_str().parse().unwrap();
        Button{ x: x_increment, y: y_increment }
    }

    fn parse_prize_line(line: &str) -> Prize {
        lazy_static! {
            static ref PRIZE: regex::Regex = Regex::new(r"^Prize: X=(?<x>\d+), Y=(?<y>\d+)$").unwrap();
        };
        let captures = PRIZE.captures(line).unwrap();
        let x = captures.name("x").unwrap().as_str().parse().unwrap();
        let y = captures.name("y").unwrap().as_str().parse().unwrap();
        Prize{x, y}
    }

    #[test]
    fn preamble() {
        let input = parse_input(PREAMBLE);
        let left = fewest_tokens(&input);
        let right = 480;
        assert_eq!(left, right)
    }
    #[test]
    fn easy() {
        let input = parse_input(EASY);
        let left = fewest_tokens(&input);
        let right = 33921;
        assert_eq!(left, right)
    }

    #[test]
    fn preamble_hard() {

        let input = parse_hard_input(PREAMBLE);
        let left = fewest_tokens(&input);
        let right = 480;
        assert_eq!(left, right)
    }
    #[test]
    fn hard() {
        let input = parse_hard_input(EASY);
        let left = fewest_tokens(&input);
        let right = 82261957837868;
        assert_eq!(left, right)
    }
}