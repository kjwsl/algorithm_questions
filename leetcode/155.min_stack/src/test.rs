use std::fmt::Display;

pub struct Test {
    test_num: u16,
}

impl Test {
    pub fn new() -> Test {
        Test { test_num: 1 }
    }
    pub fn validate<T>(&mut self, output: T, expected: T) -> bool
    where
        T: PartialEq + Display,
    {
        let success = output == expected;
        println!(
            "Test #{}: {}",
            self.test_num,
            if success { "Success" } else { "Failed" }
        );

        if !success {
            println!("Expected: {}, Got: {}", expected, output);
        }
        self.test_num += 1;
        success
    }
}
