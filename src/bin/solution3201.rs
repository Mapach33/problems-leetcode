use std::cmp::max;
struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut dp = [1; 2];
        let mut res = 1;
        for i in 1..n {
            let mut new_dp = dp;
            for parity in 0..1 {
                for j in 0..i {
                    if (nums[j] + nums[i]) % 2 == parity {
                        new_dp[parity as usize] =
                            max(new_dp[parity as usize], dp[parity as usize] + 1);
                    }
                }
            }
            dp = new_dp;
            res = max(res, max(dp[0], dp[1]));
        }

        res
    }
}

fn main() {
    let numeros = vec![1, 8, 4, 2, 4];

    let result = Solution::maximum_length(numeros);
    println!("resultado: {result}")
}
