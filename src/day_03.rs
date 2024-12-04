use lazy_static::lazy_static;
use regex::Regex;

pub fn get_computer_instructions_sum(instructions: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    };

    RE.captures_iter(instructions)
        .map(|cap| {
            let first: usize = cap.get(1).unwrap().as_str().parse().unwrap();
            let second: usize = cap.get(2).unwrap().as_str().parse().unwrap();
            first * second
        })
        .sum()
}

enum Toggle {
    On,
    Off,
}

impl From<&str> for Toggle {
    fn from(value: &str) -> Self {
        match value {
            "do" => Toggle::On,
            "don't" => Toggle::Off,
            _ => panic!(),
        }
    }
}
pub fn get_toggled_computer_instructions_sum(instructions: &str) -> usize {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<toggle>don't|do)|mul\((?P<first>\d{1,3}),(?P<second>\d{1,3})\)")
                .unwrap();
    };

    let mut toggle = Toggle::On;
    let mut total = 0;

    RE.captures_iter(instructions).for_each(|cap| {
        let toggle_match = cap.name("toggle");
        match toggle_match {
            None => {
                if let Toggle::On = toggle {
                    let first: usize = cap.name("first").unwrap().as_str().parse().unwrap();
                    let second: usize = cap.name("second").unwrap().as_str().parse().unwrap();
                    total += first * second;
                }
            }
            Some(toggle_match) => {
                toggle = Toggle::from(toggle_match.as_str());
            }
        }
    });
    total
}
#[cfg(test)]
mod tests {
    use crate::answers::{DAY_03_EASY, DAY_03_HARD};
    use crate::day_03::{get_computer_instructions_sum, get_toggled_computer_instructions_sum};

    const INSTRUCTIONS: &str = include_str!("../resources/day_03/easy.txt");
    #[test]
    fn easy() {
        let left = get_computer_instructions_sum(INSTRUCTIONS);
        let right = DAY_03_EASY;
        assert_eq!(left, right)
    }

    #[test]
    fn hard() {
        let left = get_toggled_computer_instructions_sum(INSTRUCTIONS);
        let right = DAY_03_HARD;
        assert_eq!(left, right)
    }
}
