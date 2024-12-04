type Report = Vec<isize>;

pub fn find_number_of_safe_reports(reports: &[Report]) -> isize {
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

        if !(1..4).contains(&abs_diff) {
            return false;
        }

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

pub fn find_number_of_dampened_safe_reports(reports: &[Report]) -> isize {
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

#[cfg(test)]
mod tests {
    use crate::answers::{DAY_02_EASY, DAY_02_HARD};
    use crate::day_02::{
        find_number_of_dampened_safe_reports, find_number_of_safe_reports, Report,
    };

    const INPUT: &str = include_str!("../resources/day_02/easy.txt");

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
        let left = find_number_of_dampened_safe_reports(&reports);

        let right = DAY_02_HARD;
        assert_eq!(left, right);
    }
}
