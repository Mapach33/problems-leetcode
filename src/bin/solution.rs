struct Solution;
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }
        let mut result = Vec::with_capacity(digits.len() + 1);
        result.push(1);
        result.extend(digits);
        result
    }
}

fn main() {
    let digits = vec![9, 9, 9];
    println!("{:?}", Solution::plus_one(digits));
}
