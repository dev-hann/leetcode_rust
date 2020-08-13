struct Solution {}

fn main() {
    let res = Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]);
}


impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i != j  && i<j{
                    if nums[i] == nums[j] {
                       cnt+=1;

                    }
                }
            }
        }
       cnt
    }
}