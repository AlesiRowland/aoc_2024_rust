use std::collections::VecDeque;
use std::num::ParseIntError;

struct Equation{
    result: isize,
    operators: VecDeque<isize>,
}

fn solve(mut operators: &VecDeque<isize>, expected: isize) -> bool {
    // How solve?



    let mut operators = operators.clone();
    let mut results= VecDeque::new();
    let first = operators.pop_front().unwrap();

    results.push_front(first);

    loop {
        let Some(next_op) = operators.pop_front() else {
            return results.into_iter().any(|r| r == expected)
        };

        let n = results.len();

        for _ in 0..n {
            let prev_op = results.pop_front().unwrap();

            let add_result = prev_op + next_op;
            let mul_result = prev_op * next_op;

            if add_result <= expected {
                results.push_back(add_result);
            }
            if mul_result <= expected {
                results.push_back(mul_result);
            }
        }
    }
}

fn solve_hard(mut operators: &VecDeque<isize>, expected: isize) -> bool {
    // How solve?



    let mut operators = operators.clone();
    let mut results= VecDeque::new();
    let first = operators.pop_front().unwrap();

    results.push_front(first);

    loop {
        let Some(next_op) = operators.pop_front() else {
            return results.into_iter().any(|r| r == expected)
        };

        let n = results.len();

        for _ in 0..n {
            let prev_op = results.pop_front().unwrap();

            let add_result = prev_op + next_op;
            let mul_result = prev_op * next_op;
            let concat_result = concat(prev_op, next_op).unwrap();

            if concat_result <= expected {
                results.push_back(concat_result)
            }

            if add_result <= expected {
                results.push_back(add_result);
            }
            if mul_result <= expected {
                results.push_back(mul_result);
            }
        }
    }
}

fn concat(left: isize, right: isize) -> Result<isize, ParseIntError> {
    (left.to_string() + &right.to_string()).parse()
}
fn find_sum(input: &mut Vec<(VecDeque<isize>, isize)>) -> isize {
    input
        .iter_mut()
        .filter_map(|(operators, expected)| {
            if solve(operators, *expected) {
                Some(*expected)
            } else {
                None
            }
        }).sum()

}

fn find_sum_hard(input: &mut Vec<(VecDeque<isize>, isize)>) -> isize {
    input
        .iter_mut()
        .filter_map(|(operators, expected)| {
            if solve_hard(operators, *expected) {
                Some(*expected)
            } else {
                None
            }
        }).sum()


}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use crate::answers::{DAY_07_EASY, DAY_07_HARD};
    use crate::day_07::{find_sum, find_sum_hard, Equation};

    const INPUT: &str = include_str!("../resources/day_07/day_07.txt");
    fn parse_input(input: &str) -> Vec<(VecDeque<isize>, isize)> {
        input.split("\n").map(|line| {
            let (result, operators) = line.split_once(":").unwrap();
            let result = result.parse().unwrap() ;
            let operators = operators.split_whitespace().map(|n| n.parse().unwrap()).collect();
            (operators, result)
        }).collect()
    }
    #[test]
    fn easy() {
        let mut lines = parse_input(INPUT);
        let left = find_sum(&mut lines);
        assert_eq!(left, DAY_07_EASY)

    }
    #[test]
    fn hard() {
        let mut lines = parse_input(INPUT);
        let left = find_sum_hard(&mut lines);
        assert_eq!(left, DAY_07_HARD)

    }
}