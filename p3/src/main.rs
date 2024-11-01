mod part1;
mod part2;

fn read_file(file_name: &str) -> String {
    match std::fs::read_to_string(file_name) {
        Ok(txt) => txt,
        Err(_) => {
            eprintln!("Error reading file: {}", file_name);
            std::process::exit(1);
        }
    }
}

fn main() {
    let txt = read_file("sample.txt");

    // println!("{}", part1::solve(&txt));
    println!("{}", part2::solve(&txt));
}
