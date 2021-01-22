use std::collections::HashMap;

fn main() {
    let res = Solution::single_number(vec![4, 1, 2, 1, 2]);
    println!("{}", res);
}
// https://leetcode.com/problems/single-number/

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for n in nums {
            result = result ^ n;
        }
        result
    }
}
