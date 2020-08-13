struct Solution {}

fn main() {
    let sol = Solution::running_sum(vec![1, 2, 3,4]);
    println!("{:?}", sol)
}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        res.push(nums[0]);
        for i in 1..nums.len() {
            res.push(nums[i]+res[i-1]);
        }
        res
    }
}