struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.chars().rev().collect::<Vec<char>>();
        let b = b.chars().rev().collect::<Vec<char>>();
        let mut carry = 0;

        let mut ans = String::new();
        let mut i = 0;

        while i < a.len() || i < b.len() {
            let mut sum = carry;
            if i < a.len() {
                sum += a[i].to_digit(10).unwrap();
            }
            if i < b.len() {
                sum += b[i].to_digit(10).unwrap();
            }

            carry = sum / 2;
            ans.push_str(&(sum % 2).to_string());

            i += 1;
        }

        if carry > 0 {
            ans.push_str(&carry.to_string());
        }

        ans.chars().rev().collect()
    }
}
