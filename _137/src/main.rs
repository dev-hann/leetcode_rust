use std::collections::HashMap;

fn main() {
    let res = Solution::single_number(vec![2,2,3,2]);
    println!("{}",res);
}
struct Solution;


// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Single Number II.
// Memory Usage: 2.3 MB, less than 30.43% of Rust online submissions for Single Number II.
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut _table:HashMap<i32,i32> = HashMap::new();
        for i in nums.iter(){
            match _table.get(i){
                Some(value)=>_table.insert(*i,value+1),
                None=>_table.insert(*i,1)
            };
        }
        for (key,value) in _table.iter(){
            if *value==1{
                return *key;
            }
        }
        return 1;
    }

}

