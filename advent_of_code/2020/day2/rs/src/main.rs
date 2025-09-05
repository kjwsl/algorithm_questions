use std::fs;

struct PasswordSection {
    pub min: usize,
    pub max: usize,
    pub character: char,
    pub password: String
}

fn parse_input(input: &str) -> Vec<PasswordSection> {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let range = parts[0];
        let character = parts[1];
        let password = parts[2];

        let (min, max) = range.split_once("-").unwrap();
        PasswordSection {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
            character: character.chars().next().unwrap(),
            password: password.to_string()
        }
    }).collect()
}

fn part1(input: &str) -> usize {
    let passwords = parse_input(input);

    passwords.iter().filter(|p| {
        let count = p.password.chars().filter(|c| *c == p.character).count();
        count >= p.min && count <= p.max
    }).count()
}

fn part2(input: &str) -> usize {
    let passwords = parse_input(input);

    passwords.iter().filter(|p| {
        let first = p.password.chars().nth(p.min - 1).unwrap() == p.character;
        let second = p.password.chars().nth(p.max - 1).unwrap() == p.character;
        first ^ second
    }).count()
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
