use std::collections::HashSet;

pub fn solve(input: &str) -> i32 {
    const N: usize = 4;
    for i in 0..input.len() - N {
        let slice = &input[i..i + N].chars().collect::<Vec<char>>();
        if slice.iter().collect::<HashSet<_>>().len() == N {
            return (i + N) as i32;
        }
    }
    -1
}
