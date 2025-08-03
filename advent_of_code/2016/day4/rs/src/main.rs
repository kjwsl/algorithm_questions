use std::collections::HashMap;

#[derive(Debug, Clone, Hash)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn new(name: String, sector_id: u32, checksum: String) -> Self {
        Self {
            name,
            sector_id,
            checksum,
        }
    }

    fn is_valid(&self) -> bool {
        let mut map = HashMap::new();

        for c in self.name.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        map.remove(&'-');

        let mut sorted = map.iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        sorted[0..5]
            .iter()
            .map(|&(c, _)| *c)
            .eq(self.checksum.chars())
    }

    fn decrypt_name(&self) -> String {
        let mut new_name = self.name.clone().into_bytes();

        for c in new_name.iter_mut() {
            if *c == b'-' {
                *c = b' ';
            } else {
                *c = (*c - b'a' + (self.sector_id % 26) as u8) % 26 + b'a';
            }
        }

        new_name.iter().map(|&c| c as char).collect()
    }
}

fn parse_input(input: &str) -> Vec<Room> {
    let lines = input.lines();
    let mut rooms = vec![];

    for line in lines {
        let mut parts: Vec<_> = line.split('-').collect();
        let last_part = parts.pop().unwrap();

        let name = parts.join("-");

        let bracket_idx = last_part.find('[').unwrap();
        let sector_id = last_part[0..bracket_idx].parse().unwrap();
        let checksum = last_part[bracket_idx + 1..last_part.len() - 1].to_string();

        rooms.push(Room::new(name, sector_id, checksum));
    }

    rooms
}
fn part1(input: &str) -> u32 {
    let rooms = parse_input(input);
    rooms
        .iter()
        .filter(|r| r.is_valid())
        .map(|r| r.sector_id)
        .sum()
}

fn part2(input: &str) -> u32 {
    let rooms = parse_input(input);
    
    for room in rooms.iter().filter(|r| r.is_valid()) {
        let decrypted = room.decrypt_name();
        if decrypted.contains("north") {
            return room.sector_id;
        }
    }
    
    0
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
