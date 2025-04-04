use std::{
    cmp::{self, Reverse},
    collections::BinaryHeap,
};

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

    pub fn part2(input: &str) -> i64 {
        let mut decoded_map = DecodedMap::decode(input).expect("Failed to decode input");
        println!("Decoded map: {:#?}", decoded_map);
        decoded_map.defragment();
        println!("Decoded map: {:#?}", decoded_map);
        decoded_map.checksum()
    }
}

#[derive(Debug, Clone)]
struct Chunk {
    pub pos: usize,
    pub len: usize,
}

impl Chunk {
    pub fn new(pos: usize, len: usize) -> Self {
        Self { pos, len }
    }
}

#[derive(Debug, Clone)]
struct DecodedMap {
    pub files: Vec<(u32, Chunk)>,
    pub spaces: Vec<Chunk>,
}

impl DecodedMap {
    pub fn new() -> Self {
        Self {
            files: vec![],
            spaces: vec![],
        }
    }

    pub fn decode(input: &str) -> Option<Self> {
        let input = input.trim();
        if input.is_empty() {
            return None;
        }

        let mut files = vec![];
        let mut spaces = vec![];

        let mut curr_id = 0;
        let mut is_id = true;

        for (i, ch) in input.chars().enumerate() {
            let count = match ch.to_digit(10) {
                Some(count) => count as usize,
                None => {
                    return None;
                }
            };
            if is_id {
                files.push((curr_id, Chunk::new(i, count)));
                curr_id += 1;
            } else {
                spaces.push(Chunk::new(i, count));
            }
            is_id = !is_id;
        }

        Some(Self { files, spaces })
    }

    pub fn defragment(&mut self) {
        // Start from the back of the files
        for i in (0..self.files.len()).rev() {
            for j in 0..self.spaces.len() {
                // move_file_to_space can remove number of spaces
                if self.files.get(i).is_none()
                    || self.spaces.get(j).is_none()
                    || self.files[i].1.pos < self.spaces[j].pos
                {
                    break;
                }
                // If the space is big enough to contain the file
                if self.spaces[j].len >= self.files[i].1.len {
                    self.move_file_to_space(i, j)
                        .expect("Something went wrong during defragmentation process");
                }
            }
        }
    }

    fn move_file_to_space(
        &mut self,
        file_idx: usize,
        space_idx: usize,
    ) -> Result<(), &'static str> {
        let file = match self.files.get_mut(file_idx) {
            Some(file) => file,
            None => {
                return Err("Invalid file index");
            }
        };
        let space = match self.spaces.get_mut(space_idx) {
            Some(space) => space,
            None => {
                return Err("Invalid space index");
            }
        };

        match file.1.len.cmp(&space.len) {
            cmp::Ordering::Greater => {
                return Err("No space big enough to accomodate file");
            }
            cmp::Ordering::Equal => {
                let space = self.spaces.remove(space_idx);
                file.1.pos = space.pos;
            }
            cmp::Ordering::Less => {
                file.1.pos = space.pos;
                space.pos += file.1.len;
                space.len -= file.1.len;
            }
        }

        self.files.sort_by(|a, b| a.1.pos.cmp(&b.1.pos));

        Ok(())
    }

    pub fn checksum(&self) -> i64 {
        self.files
            .iter()
            .enumerate()
            .map(|(i, file)| i as i64 * file.0 as i64)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let sample = "12345";
        // "0..111....22222"
        // 022111222
        assert_eq!(Solution::part1(sample), 1928);
    }
}
