pub struct Test;

impl Test {
    pub fn new() -> Self {
        Test
    }

    pub fn validate(&mut self, actual: i32, expected: i32) {
        if actual == expected {
            println!("✅ Test passed: {} == {}", actual, expected);
        } else {
            println!("❌ Test failed: {} != {}", actual, expected);
        }
    }
}