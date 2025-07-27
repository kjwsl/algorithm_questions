use std::collections::HashSet;

struct IPv7 {
    nets: Vec<String>,
    hypernets: Vec<String>,
}

impl IPv7 {
    pub fn new(nets: Vec<String>, hypernets: Vec<String>) -> Self {
        Self { nets, hypernets }
    }

    pub fn supports_tls(&self) -> bool {
        fn has_abba(s: &String) -> bool {
            for chunk in s.to_string().chars().collect::<Vec<char>>().windows(4) {
                if chunk[0] == chunk[3] && chunk[1] == chunk[2] && chunk[0] != chunk[1] {
                    return true;
                }
            }
            false
        }

        self.nets.iter().any(has_abba) && !self.hypernets.iter().any(has_abba)
    }

    pub fn supports_ssl(&self) -> bool {
        fn find_aba(s: &String) -> Vec<String> {
            let mut ret = vec![];
            for chunk in s.to_string().chars().collect::<Vec<char>>().windows(3) {
                if chunk[0] == chunk[2] && chunk[0] != chunk[1] {
                    ret.push(chunk.iter().collect());
                }
            }
            ret
        }

        let mut abas = HashSet::new();
        for net in &self.nets {
            abas.extend(find_aba(net));
        }

        for hypernet in &self.hypernets {
            for aba in &abas {
                let bab = format!(
                    "{}{}{}",
                    aba.chars().nth(1).unwrap(),
                    aba.chars().nth(0).unwrap(),
                    aba.chars().nth(1).unwrap()
                );
                if hypernet.contains(&bab) {
                    return true;
                }
            }
        }
        false
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let mut nets = vec![];
        let mut hypernets = vec![];

        let mut in_hypernet = false;
        let mut net = String::new();
        for c in s.chars() {
            if c == '[' {
                nets.push(net.clone());
                net.clear();
                in_hypernet = true;
            } else if c == ']' {
                hypernets.push(net.clone());
                net.clear();
                in_hypernet = false;
            }

            net.push(c);
        }

        if !net.is_empty() {
            if in_hypernet {
                hypernets.push(net);
            } else {
                nets.push(net);
            }
        }

        Some(IPv7::new(nets, hypernets))
    }
}

impl std::fmt::Display for IPv7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut net_idx = 0;
        let mut hypernet_idx = 0;

        while net_idx < self.nets.len() || hypernet_idx < self.hypernets.len() {
            if net_idx < self.nets.len() {
                s.push_str(&self.nets[net_idx]);
                net_idx += 1;
            }

            if hypernet_idx < self.hypernets.len() {
                s.push_str(&self.hypernets[hypernet_idx]);
                hypernet_idx += 1;
            }
        }

        write!(f, "{}", s)
    }
}

fn parse_input(input: &str) -> Vec<IPv7> {
    let mut ret = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        ret.push(IPv7::from_str(line).unwrap());
    }

    ret
}

fn part1(input: &str) -> usize {
    let ips = parse_input(input);

    ips.iter().filter(|ip| ip.supports_tls()).count()
}

fn part2(input: &str) -> usize {
    let ips = parse_input(input);

    ips.iter().filter(|ip| ip.supports_ssl()).count()
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
