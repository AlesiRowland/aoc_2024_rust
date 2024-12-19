use std::collections::HashMap;
use std::hash::Hash;

type Stones = Vec<isize>;

fn blink_stone(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    };
    let as_string = stone.to_string();
    let length = as_string.len();
    if (length % 2) == 0 {
        let (left, right) = as_string.split_at(length / 2);
        vec![left.parse().unwrap(), right.parse().unwrap()]
    } else {
        vec![stone * 2024]
    }
}

fn blink(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones = HashMap::new();
    for (stone, count) in stones {
        let expanded = blink_stone(stone);
        for new_stone in expanded {
            let old_count = new_stones.entry(new_stone).or_insert(0);
            let old_count = old_count.clone();
            new_stones.insert(new_stone, old_count + count);
        }
    }
    new_stones
}

fn blink_stones(stones: Vec<usize>, n: usize) -> usize {
    let mut new_stones = HashMap::new();
    stones.into_iter().for_each(|stone| {
        let old_count = new_stones.entry(stone).or_insert(0);
        let old_count = old_count.clone();
        new_stones.insert(stone, old_count + 1);
    });
    for i in 0..n {
        new_stones = blink(new_stones);
    }

    new_stones.values().sum()
}


#[cfg(test)]
mod tests {
    use crate::answers::{DAY_11_EASY, DAY_11_HARD};
    use crate::day_11::blink_stones;


    #[test]
    fn preamble() {
        let left = blink_stones(vec![125, 17], 25);
        let right = 55312;
        assert_eq!(left, right)
    }
    #[test]
    fn easy() {
        let left = blink_stones(vec![27, 10647, 103, 9, 0, 5524, 4594227, 902936], 25);
        let right = DAY_11_EASY;
        assert_eq!(left, right)
    }
    #[test]
    fn hard() {
        let left = blink_stones(vec![27, 10647, 103, 9, 0, 5524, 4594227, 902936], 75);
        let right = DAY_11_HARD;
        assert_eq!(left, right)
    }
}
