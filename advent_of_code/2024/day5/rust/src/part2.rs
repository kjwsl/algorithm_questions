use std::collections::HashMap;

pub fn solve(input: &str) -> i32 {
    let mut sum = 0;
    
    // Split input into rules and lists sections
    let (rules_section, lists_section) = input.split_once("\n\n").unwrap();
    
    // Parse ordering rules
    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules_section.lines() {
        let (a, b) = rule.split_once("|").unwrap();
        let a = a.trim().parse::<i32>().unwrap();
        let b = b.trim().parse::<i32>().unwrap();
        ordering_rules.entry(a).or_default().push(b);
    }
    
    // Process each line in the lists section
    for line in lists_section.lines() {
        let nums: Vec<i32> = line
            .split(',')
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect();
            
        // Check if order is incorrect using part1's logic
        let mut is_incorrect_order = false;
        for i in (0..nums.len()).rev() {
            if let Some(possible_next_pages) = ordering_rules.get(&nums[i]) {
                if possible_next_pages.iter().any(|&next_page| {
                    nums.iter()
                        .rev()
                        .skip(nums.len() - i)
                        .any(|&page| page == next_page)
                }) {
                    is_incorrect_order = true;
                    break;
                }
            }
        }
        
        // Only process sequences that are in incorrect order
        if is_incorrect_order {
            // Create sorted version based on rules
            let mut sorted = nums.clone();
            sorted.sort_by(|&a, &b| {
                if ordering_rules.get(&a).map_or(false, |nexts| nexts.contains(&b)) {
                    std::cmp::Ordering::Greater
                } else if ordering_rules.get(&b).map_or(false, |nexts| nexts.contains(&a)) {
                    std::cmp::Ordering::Less
                } else {
                    b.cmp(&a)
                }
            });
            
            let mid_idx = sorted.len() / 2;
            sum += sorted[mid_idx];
        }
    }
    
    sum
}
