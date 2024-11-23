fn get_crates(crate_str: &Vec<&str>) -> Vec<Vec<char>> {
    let mut crates: Vec<Vec<char>> = Vec::new();

    for &layer in crate_str {
        let mut i = 1usize;
        let mut cnt = 0usize;
        while i < layer.len() {
            let ch = layer.chars().nth(i).unwrap();
            if ch.is_ascii_alphabetic() {
                if crates.get(cnt).is_none() {
                    crates.push(Vec::new());
                }
                crates[cnt].push(ch);
            }
            cnt += 1;
            i += 4;
        }
    }

    crates
}

pub fn solve(input: &str) -> String {
    let lines = input.lines().collect::<Vec<_>>();

    let crate_str = lines[0..8].iter().rev().copied().collect::<Vec<_>>();
    let instructions = &lines[10..];

    let mut crates = get_crates(&crate_str);

    for &inst in instructions {
        let nums = inst
            .split_whitespace()
            .filter_map(|e| e.parse::<i32>().ok())
            .collect::<Vec<_>>();
        if nums.len() != 3 {
            panic!("What the heeeeeeeell");
        }

        let [num, src, dst] = nums[..3] else {
            panic!("wtf")
        };

        let mut new_val = Vec::new();
        for _ in 0..num {
            if let Some(val) = crates[src as usize - 1].pop() {
                new_val.push(val);
            } else {
                panic!("wtf");
            }
        }
        while let Some(val) = new_val.pop() {
            crates[dst as usize - 1].push(val);
        }
    }

    println!("{crates:#?}");

    crates
        .into_iter()
        .fold(String::new(), |s, mut c| s + &c.pop().unwrap().to_string())
}