pub struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut lines: Vec<Vec<String>> = Vec::new();
        lines.push(Vec::new());

        words.iter().for_each(|word| {
            let mut i = lines.len() - 1;

            let anticipated_len = lines[i].iter().map(|s| s.len() + 1).sum::<usize>() + word.len();
            if anticipated_len > max_width as usize {
                lines.push(Vec::new());
                i += 1;
            }
            lines[i].push(word.to_owned());
        });

        lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                if i == lines.len() - 1 {
                    let mut new_string = line.join(" ");
                    new_string.push_str(&" ".repeat(max_width as usize - new_string.len()));
                    return new_string;
                }

                if line.len() == 1 {
                    let mut new_string = line[0].clone();
                    new_string.push_str(&" ".repeat(max_width as usize - new_string.len()));
                    return new_string;
                }

                let total_str_len: usize = line.iter().map(|s| s.len()).sum();
                let free_space_len = max_width as usize - total_str_len;
                let space_slots = line.len() - 1;
                let space_len = free_space_len / space_slots;
                let extra_spaces = free_space_len % space_slots;

                line.iter()
                    .enumerate()
                    .fold(String::new(), |mut acc, (j, word)| {
                        if j > 0 {
                            acc.push_str(
                                &" ".repeat(space_len + if j <= extra_spaces { 1 } else { 0 }),
                            );
                        }
                        acc.push_str(word);
                        acc
                    })
            })
            .collect()
    }
}
