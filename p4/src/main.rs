mod part1;
mod part2;
mod test;
mod utils;

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

    let test_area = utils::TestArea::new(
        (200000000000000, 400000000000000),
        (200000000000000, 400000000000000),
    );
    println!("{}", part1::solve(&txt, &test_area));
    // println!("{}", part2::solve(&txt, &test_area));
}
