use std::collections::HashMap;

fn main() {
        let res = Solution::find_the_difference(String::from("abcd"),String::from("abcde"));
    println!("{}",res)
}

struct Solution;
Not Yet Solve
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut char_table:HashMap<char,i32> = HashMap::new();
        for c in t.chars(){
            char_table.insert(c,);
        }
        for c in s.chars(){
            char_table.insert(c,false);
        }
        for (key,value) in char_table{
            if value{
                return key
            }
        }
        return 'c';
    }
}