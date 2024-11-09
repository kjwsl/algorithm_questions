/// https://leetcode.com/problems/simplify-path/description/?envType=study-plan-v2&envId=top-interview-150
///
mod test;
use test::Test;

fn solve(path: String) -> String {
    if path.is_empty() {
        return path;
    }

    let mut stack = Vec::new();
    let mut parts = path.split('/').collect::<Vec<_>>();

    for part in &mut parts {
        if part.is_empty() || *part == "." {
            continue;
        }
        if *part == ".." {
            if !stack.is_empty() {
                stack.pop();
            }
            continue;
        }

        while let Some(new_str) = &mut part.strip_prefix('/') {
            *part = new_str;
        }

        stack.push(*part);
    }

    format!("/{}", stack.join("/"))
}
fn main() {
    let mut test = Test::new();
    test.validate(solve("/home/".to_string()), "/home".to_string());
    test.validate(solve("/home//foo/".to_string()), "/home/foo".to_string());
    test.validate(
        solve("/home/user/Documents/../Pictures".to_string()),
        "/home/user/Pictures".to_string(),
    );
    test.validate(solve("/../".to_string()), "/".to_string());
    test.validate(
        solve("/.../a/../b/c/../d/./".to_string()),
        "/.../b/d".to_string(),
    );
}
