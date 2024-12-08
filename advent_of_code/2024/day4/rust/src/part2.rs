fn is_xmas(map: &[Vec<char>], apos: (usize, usize)) -> bool {
    let mut count = 0;
    let (x, y) = apos;

    if x < 1 || y < 1 || x + 1 >= map.len() || y + 1 >= map[x].len() {
        return false;
    }

    let (a, b, c, d) = (
        map[x - 1][y - 1],
        map[x - 1][y + 1],
        map[x + 1][y - 1],
        map[x + 1][y + 1],
    );

    const VALID_CASES: [[char; 4]; 4] = [
        ['M', 'M', 'S', 'S'],
        ['S', 'S', 'M', 'M'],
        ['M', 'S', 'M', 'S'],
        ['S', 'M', 'S', 'M'],
    ];

    if VALID_CASES.iter().any(|&case| case == [a, b, c, d]) {
        return true;
    }

    false
}

pub fn solve(input: &str) -> i32 {
    let mut count = 0;
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'A' && is_xmas(&map, (i, j)) {
                count += 1;
            }
        }
    }

    count
}
