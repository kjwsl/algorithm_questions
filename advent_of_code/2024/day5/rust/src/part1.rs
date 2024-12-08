use std::collections::HashMap;

pub fn solve(input: &str) -> i32 {
    let mut result = 0;
    let (section_ordering_rules, section_lists_of_pages_to_update) =
        input.split_once("\n\n").unwrap();

    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in section_ordering_rules.lines() {
        let (a, b) = rule.split_once("|").unwrap();
        let a = a.trim().parse::<i32>().unwrap();
        let b = b.trim().parse::<i32>().unwrap();
        ordering_rules.entry(a).or_default().push(b);
    }

    for update_list in section_lists_of_pages_to_update.lines() {
        let pages = update_list
            .split(",")
            .map(|page| page.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut is_correct_order = true;
        for i in (0..pages.len()).rev() {
            if let Some(possible_next_pages) = ordering_rules.get(&pages[i]) {
                if possible_next_pages.iter().any(|&next_page| {
                    pages
                        .iter()
                        .rev()
                        .skip(pages.len() - i)
                        .any(|&page| page == next_page)
                }) {
                    is_correct_order = false;
                    break;
                }
            }
        }

        if is_correct_order {
            result += pages[pages.len() / 2];
        }
    }

    result
}
