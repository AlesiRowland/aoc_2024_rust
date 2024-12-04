use std::collections::HashMap;

pub fn find_sorted_differences_sum(left: &mut [isize], right: &mut [isize]) -> isize {
    left.sort();
    right.sort();
    left.iter().zip(right).map(|(l, r)| (*l - *r).abs()).sum()
}

pub fn get_similarity_score(left: &[isize], right: &[isize]) -> isize {
    // create a list of the counts of each value to avoid running through the list alot
    let count_lookup = create_count_lookup(right);
    left.iter()
        .map(|val| *val * *count_lookup.get(val).unwrap_or(&0))
        .sum()
}

fn create_count_lookup(values: &[isize]) -> HashMap<isize, isize> {
    let mut count_lookup = HashMap::new();
    values.iter().for_each(|value| {
        let current = count_lookup.get(value).unwrap_or(&0);
        let new = *current + 1;
        count_lookup.insert(*value, new);
    });
    count_lookup
}

#[cfg(test)]
mod tests {
    use crate::answers::{DAY_01_EASY, DAY_01_HARD};
    use crate::day_01::{create_count_lookup, find_sorted_differences_sum, get_similarity_score};

    const INPUT: &str = include_str!("../resources/day_01/easy.txt");

    fn parse_input(input: &str) -> [Vec<isize>; 2] {
        let mut left = Vec::new();
        let mut right = Vec::new();
        input.split('\n').for_each(|line| {
            let mut parts = line.split("   ").map(|val| val.parse::<isize>().unwrap());
            left.push(parts.next().unwrap());
            right.push(parts.next().unwrap());
        });
        [left, right]
    }

    #[test]
    fn easy() {
        let mut lists = parse_input(INPUT);
        let left = &mut lists[0].clone();
        let right = &mut lists[1].clone();
        let left = find_sorted_differences_sum(left, right);
        let right = DAY_01_EASY;
        assert_eq!(left, right);
    }

    #[test]
    fn hard() {
        let mut lists = parse_input(INPUT);
        let left = &mut lists[0].clone();
        let right = &mut lists[1].clone();
        let left = get_similarity_score(left, right);
        let right = DAY_01_HARD;
        assert_eq!(left, right);
    }
}
