//https://leetcode.com/problems/two-sum/
fn main() {
    let s = Solution::two_sum(vec![-1, -2, -3, -4, -5]
                              , -8);
    println!("{:?}", s)
}

struct Solution {}

impl Solution {
    fn check_len(nums: &Vec<i32>) -> bool {
        if nums.len() == 2 {
            return true;
        }
        return false;
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //  check length is 2
        if Solution::check_len(&nums) {
            return vec![0, 1];
        }
        let len = nums.len();
        for i in 0..len {
            for j in 0..len {
                if i == j {
                    continue;
                }
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        unreachable!()
    }
}