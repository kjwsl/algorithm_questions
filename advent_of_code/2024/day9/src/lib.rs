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
        decoded_map.defragment();
        println!("Decoded map: {}", decoded_map.to_str_repr());
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
        let mut idx = 0;

        for ch in input.chars() {
            let count = match ch.to_digit(10) {
                Some(count) => count as usize,
                None => {
                    return None;
                }
            };

            // No need to push empty files/spaces
            if count > 0 {
                if is_id {
                    files.push((curr_id, Chunk::new(idx, count)));
                    curr_id += 1;
                } else {
                    spaces.push(Chunk::new(idx, count));
                }
            }

            idx += count;
            is_id = !is_id;
        }

        Some(Self { files, spaces })
    }

    pub fn defragment(&mut self) {
        // Sort by id
        self.files.sort_unstable_by_key(|f| f.0);

        // Start from the back of the files
        for i in (0..self.files.len()).rev() {
            for j in 0..self.spaces.len() {
                if self.spaces[j].pos >= self.files[i].1.pos {
                    continue;
                }
                // If the space is big enough to contain the file
                if self.spaces[j].len >= self.files[i].1.len {
                    self.move_file_to_space(i, j)
                        .expect("Something went wrong during defragmentation process");
                }
            }
        }
    }

    pub fn checksum(&self) -> i64 {
        self.files
            .iter()
            .map(|f| {
                (0..f.1.len)
                    .map(|i| (f.1.pos + i) as i64 * f.0 as i64)
                    .sum::<i64>()
            })
            .sum()
    }

    pub fn to_str_repr(&self) -> String {
        let n = self.files.iter().map(|f| f.1.len).sum::<usize>()
            + self.spaces.iter().map(|s| s.len).sum::<usize>();
        let mut result = vec![String::new(); n];

        for file in &self.files {
            let num_str = file.0.to_string();
            let len = file.1.len;
            let pos = file.1.pos;
            for i in 0..len {
                result[pos + i] = num_str.clone();
            }
        }

        for space in &self.spaces {
            let len = space.len;
            let pos = space.pos;
            for i in 0..len {
                result[pos + i] = ".".to_string();
            }
        }

        result.join(", ")
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
                let og_file_pos = file.1.pos;
                file.1.pos = space.pos;
                space.len = 0;
                self.spaces.push(Chunk::new(og_file_pos, file.1.len));
            }
            cmp::Ordering::Less => {
                let og_space_pos = space.pos;

                // Remove the front part of the space
                space.pos += file.1.len;
                space.len -= file.1.len;

                // The spaces that the file originally occupied are now empty spaces
                self.spaces.push(Chunk::new(file.1.pos, file.1.len));

                // Move the file to the space
                file.1.pos = og_space_pos;
            }
        }

        Ok(())
    }

    fn sort_and_merge(&mut self) {
        self.files.sort_unstable_by_key(|f| f.1.pos);

        self.spaces.retain(|space| space.len > 0);

        self.spaces.sort_unstable_by_key(|s| s.pos);

        // Merge adjacent spaces
        let mut i = 0;
        while i < self.spaces.len().saturating_sub(1) {
            let curr_end = self.spaces[i].pos + self.spaces[i].len;
            let next_start = self.spaces[i + 1].pos;

            if curr_end >= next_start {
                let next_end = self.spaces[i + 1].pos + self.spaces[i + 1].len;
                self.spaces[i].len = next_end.saturating_sub(self.spaces[i].pos);
                self.spaces.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::part1(SAMPLE), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::part2(SAMPLE), 2858);
    }
}
