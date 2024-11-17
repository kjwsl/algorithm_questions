mod m1;
mod m2;

fn main() {
    test(m1::Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    test(m1::Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    test(m1::Solution::search_insert(vec![1, 3, 5, 6], 7), 4);

    test(m2::Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    test(m2::Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    test(m2::Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
}

fn test<T>(received: T, expected: T)
where
    T: std::cmp::PartialEq + std::fmt::Debug,
{
    let result = received == expected;
    match result {
        true => println!("Pass"),
        false => println!("Fail: received={:?}, expected={:?}", received, expected),
    }
}
