#[derive(Debug, Clone)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Self {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn has_required_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_valid(&self) -> bool {
        self.has_required_fields()
            && self
                .birth_year
                .as_ref()
                .and_then(|byr| byr.parse::<i32>().ok())
                .is_some_and(|byr| (1920..=2002).contains(&byr))
            && self
                .issue_year
                .as_ref()
                .and_then(|iyr| iyr.parse::<i32>().ok())
                .is_some_and(|iyr| (2010..=2020).contains(&iyr))
            && self
                .expiration_year
                .as_ref()
                .and_then(|eyr| eyr.parse::<i32>().ok())
                .is_some_and(|eyr| (2020..=2030).contains(&eyr))
            && self.height.as_ref().is_some_and(|hgt| {
                let n = hgt.len();
                let value = hgt[..n - 2].parse::<i32>().ok();
                match &hgt[n - 2..] {
                    "cm" => value.is_some_and(|cm| (150..=193).contains(&cm)),
                    "in" => value.is_some_and(|inches| (59..=76).contains(&inches)),
                    _ => false,
                }
            })
            && self.hair_color.as_ref().is_some_and(|hcl| {
                hcl.len() == 7
                    && hcl.starts_with('#')
                    && hcl[1..].chars().all(|c| c.is_ascii_hexdigit())
            })
            && self.eye_color.as_ref().is_some_and(|ecl| {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str())
            })
            && self
                .passport_id
                .as_ref()
                .is_some_and(|pid| pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit()))
    }
}

fn parse_input(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|p| {
            let mut passport = Passport::new();
            p.replace("\n", " ")
                .split_ascii_whitespace()
                .for_each(|field| {
                    let (key, value) = field.split_once(':').unwrap();
                    match key {
                        "byr" => passport.birth_year = Some(value.to_string()),
                        "iyr" => passport.issue_year = Some(value.to_string()),
                        "eyr" => passport.expiration_year = Some(value.to_string()),
                        "hgt" => passport.height = Some(value.to_string()),
                        "hcl" => passport.hair_color = Some(value.to_string()),
                        "ecl" => passport.eye_color = Some(value.to_string()),
                        "pid" => passport.passport_id = Some(value.to_string()),
                        "cid" => passport.country_id = Some(value.to_string()),
                        _ => panic!("Unknown field {}", key),
                    }
                });
            passport
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let passports = parse_input(input);
    passports.iter().filter(|p| p.has_required_fields()).count()
}

fn part2(input: &str) -> usize {
    let passports = parse_input(input);
    passports.iter().filter(|p| p.is_valid()).count()
}

fn main() {
    let filepath = match std::env::args().nth(1) {
        Some(filepath) => filepath,
        None => "../input.txt".to_string(),
    };
    let input = std::fs::read_to_string(filepath).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
