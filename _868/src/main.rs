fn main() {
    let res = Solution::binary_gap(5);
    println!("{}", res);
}

struct Solution;

try binary logic!
impl Solution {

    pub fn binary_gap(n: i32) -> i32 {
        let _binary_value = format!("{:b}", n);
        let mut res =0;
        let mut pre_index:usize=0;
        for (index,value) in _binary_value.chars().enumerate(){
            if value=='1'{
                let diff = index- pre_index;
                if res<diff{
                    res = diff
                }
                pre_index = index;
            }
        }
        return res as i32;
    }
}




// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Binary Gap.
// Memory Usage: 2 MB, less than 33.33% of Rust online submissions for Binary Gap.
/*
impl Solution {

    pub fn binary_gap(n: i32) -> i32 {
        let _binary_value = format!("{:b}", n);
        let mut res =0;
        let mut pre_index:usize=0;
        for (index,value) in _binary_value.chars().enumerate(){
            if value=='1'{
                let diff = index- pre_index;
                if res<diff{
                    res = diff
                }
                pre_index = index;
            }
        }
        return res as i32;
    }
}
*/
