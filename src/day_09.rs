use crate::common::Point;
use std::collections::VecDeque;

type Block = Option<usize>;
fn parse_input(input: &str) -> Vec<Block> {
    let mut id = 0;
    let mut blocks = Vec::new();

    let mut is_block = true;

    input.chars().for_each(|ch| {
        if is_block {
            let n = ch.to_digit(10).unwrap();
            for _ in 0..n {
                blocks.push(Block::Some(id));
            }
            is_block = false;
            id += 1
        } else {
            let n = ch.to_digit(10).unwrap();
            for _ in 0..n {
                blocks.push(Block::None)
            }
            is_block = true;
        }
    });
    blocks
}

trait Compress {
    fn compress_blocks(&mut self);
    fn compress_files(&mut self);
}

impl Compress for [Block] {
    fn compress_blocks(&mut self) {
        let mut i = 0;
        let mut stop = self.len() - 1;
        while i < stop {
            match self[i] {
                Some(_) => {
                    i += 1;
                    continue;
                }
                None => {
                    // Skip all the dots
                    while let None = self[stop] {
                        stop -= 1;
                    }

                    while stop > i {
                        match self[stop] {
                            Some(_) => {
                                let last = self[stop].take().unwrap();
                                self[i] = Some(last);
                                stop -= 1;
                                break;
                            }
                            None => {
                                stop -= 1;
                            }
                        }
                    }
                }
            }
        }
    }

    fn compress_files(&mut self) {

        let mut last_index = self.len() -1;
        loop {
            let Some(file_stop) = find_previous_file_stop(self, last_index) else {
                return;
            };
            let file_start = find_file_start(self, file_stop);

            try_move_file_stopping_at(self, file_start, file_stop);
            let Some(next_index) = file_start.checked_sub(1) else {
                return
            };
            last_index = next_index;
        }


    }
}

// failures
// it either finds no file
fn try_move_file_stopping_at(blocks: &mut [Block], file_start: usize, file_stop: usize) {

    let file_length = file_stop - file_start + 1; // add one because a file of length one's diff will be zero
    let mut index = 0;
    loop {
        let Some(free_space_start) = find_next_free_space_start(blocks, index) else {
            break
        };

        if free_space_start > file_start {
            break
        }
        let free_space_stop = find_free_space_stop(blocks, free_space_start);
        let free_space_length = free_space_stop - free_space_start+ 1;
        if file_length <= free_space_length {
            let Some(file_id) = blocks[file_start] else {panic!()};
            // you should be using file_length
            for i in free_space_start..(free_space_start + file_length) {
                blocks[i] = Some(file_id)
            }
            for i in file_start..file_stop + 1 {
                blocks[i] = None;
            }
            break
        } else {
            // find the next one starting it free space stop + 1
            index = free_space_stop + 1;
            if index >= blocks.len() {
                break
            }
        }
    }

}

fn find_previous_file_stop(blocks: &[Block], from: usize) -> Option<usize> {
    let mut from = from;
    while let None = blocks[from] {
        from = from.checked_sub(1)?
    }
    Some(from)
}

fn find_next_free_space_start(blocks: &[Block], from: usize) -> Option<usize> {
    let mut free_space_start = from;
    while let Some(_) = blocks[free_space_start] {
        free_space_start = free_space_start.checked_add(1).unwrap();
        if free_space_start >= blocks.len() {
            return None
        }
    }
    Some(free_space_start)
}


fn find_file_start(blocks: &[Block], file_stop: usize) -> usize {
    let file_id = &blocks[file_stop].unwrap();
    let mut file_start = file_stop;

    while let Some(previous_index) = file_start.checked_sub(1) {
        let block = &blocks[previous_index];
        match block {
            Some(previous_id) if previous_id == file_id => {
                file_start -= 1;
            }
            Some(_) | None => break,
        }
    }
    file_start
}

fn find_free_space_stop(blocks: &[Block], free_space_start: usize) -> usize {
    let mut free_space_stop = free_space_start;
    while let Some(next_index) = free_space_stop.checked_add(1) {
        if next_index == blocks.len() {
            break;
        }

        match blocks[next_index] {
            Some(_) => break,
            None => {
                free_space_stop += 1;
            }
        }
    }

    free_space_stop
}
#[cfg(test)]
mod tests {
    use crate::answers::{DAY_09_EASY, DAY_09_HARD};
    use crate::day_09::{find_file_start, parse_input, Compress};

    const EASY: &str = include_str!("../resources/day_09/easy.txt");
    const PREAMBLE: &str = include_str!("../resources/day_09/preamble.txt");

    #[test]
    fn preamble() {
        let mut input = parse_input(PREAMBLE);
        input.compress_blocks();
        let left = input
            .into_iter()
            .enumerate()
            .filter_map(|(index, block)| block.map(|id| id * index))
            .sum::<usize>();
        println!("{:?}", left);
    }

    #[test]
    fn easy() {
        let mut input = parse_input(EASY);
        input.compress_blocks();
        let left = input
            .into_iter()
            .enumerate()
            .filter_map(|(index, block)| block.map(|id| id * index))
            .sum::<usize>();
        let right = 6259790630969;
        assert_eq!(left, right)
    }
    #[test]
    fn preamble_hard() {
        let mut input = parse_input(PREAMBLE);
        input.compress_files();
        let left = input
            .into_iter()
            .enumerate()
            .filter_map(|(index, block)| block.map(|id| id * index))
            .sum::<usize>();

        let right = DAY_09_EASY;
        assert_eq!(left, right)
    }
    #[test]
    fn hard() {
        let mut input = parse_input(EASY);
        input.compress_files();
        let left = input
            .into_iter()
            .enumerate()
            .filter_map(|(index, block)| block.map(|id| id * index))
            .sum::<usize>();

        let right = DAY_09_HARD;
        assert_eq!(left, right)
    }
}
