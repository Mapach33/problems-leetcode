use std::cmp::max;
struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut res = 0;

        for parity in 0..=1 {
            // Empezamos siempre incluyendo el primer elemento
            let mut last = nums[0];
            let mut count = 1;

            //
            for i in nums.iter().skip(1) {
                if ((last + i) % 2) == parity {
                    count += 1;
                    last = *i; // Avanzamos el último elemento usado
                }
            }
            res = max(res, count);
        }
        res
    }
}

fn main() {
    let numeros = vec![1, 2, 1, 1, 2, 1, 2];

    let result = Solution::maximum_length(numeros);
    println!("resultado: {result}")
}
