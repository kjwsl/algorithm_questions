fn decompress(input: &str) -> String {
    let mut new_str = String::new();

    let bytes = input.as_bytes();

    let mut i = 0;
    while i < input.len() {
        match bytes[i] {
            b'(' => {
                let start_idx = i + 1;
                let mut end_idx = start_idx;
                while end_idx < input.len() && bytes[end_idx] != b')' {
                    end_idx += 1;
                }
                let marker = bytes[start_idx..end_idx]
                    .iter()
                    .map(|b| *b as char)
                    .collect::<String>();
                let (subsequent, times) = marker.split_once('x').unwrap();

                let subsequent = subsequent.parse::<usize>().unwrap();
                let times = times.parse::<usize>().unwrap();

                let sub_str = bytes[end_idx + 1..end_idx + 1 + subsequent]
                    .iter()
                    .map(|b| *b as char)
                    .collect::<String>();

                for _ in 0..times {
                    new_str.push_str(&sub_str);
                }

                i = end_idx + subsequent + 1;
            }
            _ => {
                new_str.push(bytes[i] as char);
                i += 1;
            }
        }
    }

    new_str
}

fn decompress2(input: &str) -> String {
    let mut new_str = String::new();

    let bytes = input.as_bytes();

    let mut i = 0;
    while i < input.len() {
        match bytes[i] {
            b'(' => {
                let start_idx = i + 1;
                let mut end_idx = start_idx;
                while end_idx < input.len() && bytes[end_idx] != b')' {
                    end_idx += 1;
                }
                let marker = bytes[start_idx..end_idx]
                    .iter()
                    .map(|b| *b as char)
                    .collect::<String>();
                let (subsequent, times) = marker.split_once('x').unwrap();

                let subsequent = subsequent.parse::<usize>().unwrap();
                let times = times.parse::<usize>().unwrap();

                let sub_str = bytes[end_idx + 1..end_idx + 1 + subsequent]
                    .iter()
                    .map(|b| *b as char)
                    .collect::<String>();

                for _ in 0..times {
                    new_str.push_str(&sub_str);
                }

                i = end_idx + subsequent + 1;
            }
            _ => {
                new_str.push(bytes[i] as char);
                i += 1;
            }
        }
    }

    new_str
}
fn part1(input: &str) -> usize {
    decompress(input).len()
}

fn part2(input: &str) -> usize {
    decompress2(input).len()
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}
