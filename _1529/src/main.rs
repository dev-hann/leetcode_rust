fn main() {
    let res = Solution::min_flips("001011101".to_string());
    println!("{}", res)
}

struct Solution;

// Runtime: 4 ms, faster than 100.00% of Rust online submissions for Bulb Switcher IV.
// Memory Usage: 2.2 MB, less than 100.00% of Rust online submissions for Bulb Switcher IV.
// so Easy
impl Solution {
    pub fn min_flips(target: String) -> i32 {
        if !target.contains("0") {
            return 1;
        }
        if !target.contains("1") {
            return 0;
        }
        let mut _count =0;
        let mut _flag:char='0';

        for c in target.chars(){
            if _flag!=c{
                _count+=1;
                _flag=c;
            }
        }
        return _count;
    }
}