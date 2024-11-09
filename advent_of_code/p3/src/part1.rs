use std::ops::Range;

#[inline]
pub fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

pub fn get_number_ranges(line: &str) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let mut start = 0;
    let mut in_range = false;

    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            if !in_range {
                start = i;
                in_range = true;
            }
        } else {
            if in_range {
                ranges.push(start..i);
                in_range = false;
            }
        }
    }

    if in_range {
        ranges.push(start..line.len());
    }

    ranges
}

pub fn is_part_number(lines: &[&str], curr_line_idx: usize, range: &Range<usize>) -> bool {
    let start_line_idx = if curr_line_idx > 0 {
        curr_line_idx - 1
    } else {
        0
    };
    let end_line_idx = if curr_line_idx < lines.len() - 1 {
        curr_line_idx + 1
    } else {
        lines.len() - 1
    };

    let start_col_idx = if range.start > 0 { range.start - 1 } else { 0 };
    let end_col_idx = if range.end < lines[curr_line_idx].len() - 1 {
        range.end
    } else {
        lines[curr_line_idx].len() - 1
    };

    for row_idx in start_line_idx..=end_line_idx {
        let line = &lines[row_idx];
        if row_idx == curr_line_idx {
            let start_char = line.chars().nth(start_col_idx).unwrap();
            let end_char = line.chars().nth(end_col_idx).unwrap();
            if is_symbol(start_char) || is_symbol(end_char) {
                return true;
            }
        } else {
            for j in start_col_idx..=end_col_idx {
                if is_symbol(line.chars().nth(j).unwrap()) {
                    return true;
                }
            }
        }
    }

    false
}

pub fn solve(txt: &str) -> usize {
    let lines: Vec<&str> = txt.lines().collect();
    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }
        let ranges = get_number_ranges(line);
        println!("{:?}", ranges);
        for range in ranges {
            if is_part_number(&lines, i, &range) {
                sum += &line[range.clone()].parse::<usize>().unwrap();
                println!("line:{:?}, range: {:?}, sum: {:?}", &i, &range, &sum);
            }
        }
    }

    sum
}
