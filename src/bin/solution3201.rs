use std::cmp::max;
struct Solution;
impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for parity in 0..=1 {
            let mut dp_parity = [0; 2];
            for &num in &nums {
                let num_parity = (num % 2) as usize;
                if parity == 0 {
                    dp_parity[num_parity] += 1
                } else {
                    dp_parity[num_parity] =
                        max(dp_parity[num_parity], dp_parity[1 - num_parity] + 1);
                }
            }
            res = max(res, max(dp_parity[0], dp_parity[1]));
        }
        res
    }
}

fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    println!("{}", Solution::maximum_length(numeros));
}
