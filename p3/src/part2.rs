use crate::part1::is_part_number;

use std::cmp::{max, min};
use std::{ops::Range, usize};

#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Number {
    pub value: u64,
    pub row: usize,
    pub col_range: Range<usize>,
}

impl Number {
    fn new(value: u64, row: usize, col_range: Range<usize>) -> Self {
        Number {
            value,
            row,
            col_range,
        }
    }
}

#[derive(Debug)]
struct Gear {
    position: Position,
    numbers: [u64; 2],
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

impl Gear {
    fn new(position: Position, numbers: [u64; 2]) -> Self {
        Gear { position, numbers }
    }
}

fn get_gears(lines: &[&str]) -> Vec<Gear> {
    let mut gears = Vec::new();
    for (row_num, line) in lines.iter().enumerate() {
        let mut new_line = line.to_string();
        while let Some(col_idx) = new_line.find("*") {
            new_line.replace_range(col_idx..col_idx + 1, " ");
            let start_point = Position::new(max(0, row_num - 1), max(col_idx - 1, 0));
            let end_point = Position::new(
                min(lines.len() - 1, row_num + 1),
                min(line.len() - 1, col_idx + 1),
            );

            let mut new_row_num = start_point.row;
            let mut num_cnt = 0;
            let mut gear = Gear::new(Position::new(row_num, col_idx), [0, 0]);
            while new_row_num <= end_point.row && num_cnt < 2 {
                let mut new_col_idx = start_point.col;
                while new_col_idx <= end_point.col && num_cnt < 2 {
                    if new_row_num == row_num && new_col_idx == col_idx {
                        new_col_idx += 1;
                        continue;
                    }
                    let char = lines[new_row_num].chars().nth(new_col_idx).unwrap();
                    if char.is_digit(10) {
                        if num_cnt >= 2 {
                            num_cnt += 1;
                            break;
                        }

                        let full_number = get_full_number(&lines, new_row_num, new_col_idx);
                        if !is_part_number(lines, new_row_num, &full_number.col_range) {
                            break;
                        }
                        gear.numbers[num_cnt] = full_number.value;

                        while new_col_idx < lines[new_row_num].len()
                            && lines[new_row_num]
                                .chars()
                                .nth(new_col_idx)
                                .unwrap()
                                .is_digit(10)
                        {
                            new_col_idx += 1;
                        }
                        num_cnt += 1;
                    }

                    new_col_idx += 1;
                }
                new_row_num += 1;
            }

            if num_cnt == 2 {
                gears.push(gear);
            }
        }
    }

    gears
}

fn get_full_number(lines: &[&str], row_idx: usize, col_idx: usize) -> Number {
    let mut lpivot = col_idx;
    let mut rpivot = col_idx;
    let line = lines[row_idx];

    while lpivot > 0 && line.chars().nth(lpivot - 1).unwrap().is_digit(10) {
        lpivot -= 1;
    }
    while rpivot < line.len() - 1 && line.chars().nth(rpivot + 1).unwrap().is_digit(10) {
        rpivot += 1;
    }

    let full_number = line[lpivot..=rpivot].parse::<u64>().unwrap();

    Number::new(full_number, 0, lpivot..(rpivot + 1))
}

fn get_gear_ratio(gear: &Gear, lines: &[&str]) -> u64 {
    gear.numbers[0] * gear.numbers[1]
}

pub fn solve(txt: &str) -> u64 {
    let mut sum = 0u64;
    let lines = txt.lines().collect::<Vec<&str>>();
    let gears = get_gears(&lines);
    gears
        .iter()
        .for_each(|gear| println!("Numbers: {:?}", gear.numbers));
    for (i, gear) in gears.iter().enumerate() {
        let gear_ratio = get_gear_ratio(&gear, &lines);
        sum += gear_ratio;
    }

    sum
}
