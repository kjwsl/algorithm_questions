use std::{cmp::Reverse, collections::BinaryHeap, num};

pub struct Solution;

impl Solution {
    pub fn part1(input: &str) -> i64 {
        let input = input.trim();
        let n = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .sum::<usize>();
        let mut decoded_disk_map = vec![None; n];
        let mut space_pos_list = BinaryHeap::new();

        let mut curr_id = 0i32;
        let mut is_id = true;
        let mut idx = 0;

        // Decode disk map
        for c in input.chars() {
            let num = c.to_digit(10).unwrap();

            for _ in 0..num {
                if is_id {
                    decoded_disk_map[idx] = Some(curr_id);
                } else {
                    space_pos_list.push(Reverse(idx));
                }
                idx += 1;
            }

            if is_id {
                curr_id += 1;
            }

            is_id = !is_id;
        }

        fn find_last_num_pos(disk_map: &[Option<i32>]) -> usize {
            for (i, cell) in disk_map.iter().enumerate().rev() {
                if cell.is_some() {
                    return i;
                }
            }

            0
        }

        // File empty spaces with trailing numbers
        for space_pos in space_pos_list {
            let space_pos = space_pos.0;
            let num_pos = find_last_num_pos(&decoded_disk_map);
            println!("space_pos: {}, num_pos: {}", space_pos, num_pos);
            if space_pos >= num_pos {
                break;
            }
            decoded_disk_map.swap(space_pos, num_pos);
        }

        decoded_disk_map
            .iter()
            .enumerate()
            .take_while(|(_, c)| c.is_some())
            .map(|(i, c)| i as i64 * c.unwrap() as i64)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sample1() {
        let sample = "12345";
        // "0..111....22222"
        // 022111222
        assert_eq!(Solution::part1(sample), 1928);
    }

}
