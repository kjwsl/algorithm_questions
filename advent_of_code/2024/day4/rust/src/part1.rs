fn count_xmas(map: &[Vec<char>], x: usize, y: usize) -> i32 {
    let directions = [
        (-1, 0),  // Up
        (1, 0),   // Down
        (0, -1),  // Left
        (0, 1),   // Right
        (-1, -1), // Up-Left
        (-1, 1),  // Up-Right
        (1, -1),  // Down-Left
        (1, 1),   // Down-Right
    ];

    let pattern = ['M', 'A', 'S'];
    let mut count = 0;

    for &(dx, dy) in &directions {
        let mut matched = true;
        for (k, &expected_char) in pattern.iter().enumerate() {
            let new_x = x as isize + dx * (k as isize + 1);
            let new_y = y as isize + dy * (k as isize + 1);

            // Check bounds
            if new_x < 0
                || new_x >= map.len() as isize
                || new_y < 0
                || new_y >= map[0].len() as isize
                || map[new_x as usize][new_y as usize] != expected_char
            {
                matched = false;
                break;
            }
        }
        if matched {
            count += 1;
        }
    }

    count
}

pub fn solve(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'X' {
                count += count_xmas(&map, i, j);
            }
        }
    }

    count
}
