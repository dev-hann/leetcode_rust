fn main() {
        let res = Solution::can_win_nim(4);
    println!("{}",res)
}

struct Solution;
// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Nim Game.
// Memory Usage: 2 MB, less than 75.00% of Rust online submissions for Nim Game.
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        return n%4==0;
    }
}