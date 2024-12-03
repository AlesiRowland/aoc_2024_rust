use std::cmp;
use std::cmp::PartialEq;

type Report = Vec<isize>;

fn parse_input(input: &str) -> Vec<Report> {
    input
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect()
        })
        .collect()
}

fn find_number_of_safe_reports(reports: &Vec<Report>) -> isize {
    reports
        .iter()
        .map(|report| is_safe_report(report) as isize)
        .sum()
}

fn is_safe_report(report: &Report) -> bool {
    let mut index = 1;
    let mut order = 0;

    while index < report.len() {
        let last_level = report.get(index - 1).unwrap();
        let current_level = report.get(index).unwrap();
        let diff = current_level.checked_sub(*last_level).unwrap();
        let abs_diff = diff.abs();

        if abs_diff < 1 || abs_diff > 3 {
            return false;
        };

        let current_order = diff / abs_diff;

        if order == 0 {
            order = current_order
        } else if order != current_order {
            return false;
        }

        index += 1;
    }
    true
}

fn find_number_of_dampened_safe_reports(reports: &Vec<Report>) -> isize {
    reports
        .iter()
        .map(|report| {
            let sub_reports = create_sub_reports(report);
            sub_reports.iter().any(is_safe_report) as isize
        })
        .sum()
}

fn create_sub_reports(report: &Report) -> Vec<Report> {
    (0..report.len())
        .into_iter()
        .map(|skipped| create_sub_report(skipped, report))
        .collect()
}

fn create_sub_report(skipped: usize, report: &Report) -> Report {
    let mut sub_report = Vec::new();
    report.iter().enumerate().for_each(|(index, value)| {
        if index != skipped {
            sub_report.push(*value);
        }
    });
    sub_report
}

fn find_n(reports: &Vec<Report>) -> isize {
    reports
        .iter()
        .map(|report| is_safe_enough_report(report) as isize)
        .sum()
}
fn is_safe_enough_report(report: &Report) -> bool {
    let mut last_index;
    let mut current_index = 1;

    let mut asc_count = 0;
    let mut desc_count = 0;
    let mut skipped = 0;

    while current_index < report.len() {
        last_index = current_index - 1;

        let last_level = report[last_index];
        let current_level = report[current_index];

        let diff = current_level.checked_sub(last_level).unwrap();
        let mut last_diff = 0;

        let abs_diff = diff.abs();

        // check if it is a safe distance
        let in_bounds = check_in_bounds(abs_diff);
        if in_bounds {
            if diff > 0 {
                asc_count += 1;
                last_diff = 1;
            } else {
                desc_count += 1;
                last_diff = -1
            }
        } else {
            let last_index = last_index.checked_sub(1);
            match last_index {
                None => {
                    skipped += 1;
                }
                Some(last_index) => {
                    let last_level = report[last_index];
                    let diff = current_level.checked_sub(last_level).unwrap();
                    let abs_diff = diff.abs();
                    let in_bounds = check_in_bounds(abs_diff);
                    if in_bounds {
                        if diff > 0 {
                            asc_count += 2;
                        } else {
                            desc_count += 2;
                        }
                        skipped += 1;
                    } else {
                        skipped += 2;
                    }
                }
            }
        };

        if (skipped + (current_index - cmp::max(asc_count, desc_count) as usize)) > 1 {
            return false;
        }

        current_index += 1;
    }
    true
}

fn check_in_bounds(abs_diff: isize) -> bool {
    if abs_diff > 3 || abs_diff < 1 {
        false
    } else {
        true
    }
}
#[cfg(test)]
mod tests {
    use crate::answers::{DAY_02_EASY, DAY_02_HARD};
    use crate::day_02::{
        find_n, find_number_of_dampened_safe_reports, find_number_of_safe_reports, parse_input,
    };

    const INPUT: &str = include_str!("../resources/day_02/easy.txt");

    #[test]
    fn easy() {
        let reports = parse_input(INPUT);
        let left = find_number_of_safe_reports(&reports);
        let right = DAY_02_EASY;
        assert_eq!(left, right);
    }

    #[test]
    fn hard() {
        let reports = parse_input(INPUT);
        let left = find_n(&reports);

        let right = DAY_02_HARD;
        assert_eq!(left, right);
    }
}
