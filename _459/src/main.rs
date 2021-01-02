
fn main() {

    let res: bool = Solution::repeated_substring_pattern("abab".to_string());

    println!("{}", res)
}

struct Solution {}


impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
       let len = s.len();
        if len<= 1{
            return false;
        }
        let chars:Vec<char> = s.chars().collect();
        let mut p =1;


        return false
    }
}