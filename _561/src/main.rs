fn main() {
    let res = Solution::array_pair_sum(vec![1,4,3,2]);
    println!("{}", res);
}

struct Solution {}

//Runtime: 12 ms, faster than 68.42% of Rust online submissions for Array Partition I.
// Memory Usage: 2.5 MB, less than 5.26% of Rust online submissions for Array Partition I.
/*
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut _tmp_nums:Vec<i32> = nums.clone();
        _tmp_nums.sort();
        let mut res = 0;
        for (index,i) in _tmp_nums.iter().enumerate(){
            if index%2==0{
                res+=i;
            }
        }
        return res;
    }
}

*/
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut _tmp_nums = nums;
        _tmp_nums.sort_unstable();
        _tmp_nums.iter().step_by(2).sum()
    }
}