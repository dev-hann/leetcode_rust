
//https://leetcode.com/problems/count-the-number-of-consistent-strings/
fn main() {
    let _words: Vec<String> = vec!["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"]
        .iter().map(|s| s.to_string()).collect();
    let res = Solution::count_consistent_strings("cad".to_string(), _words);
    println!("result : {}", res);
}

struct Solution {}
///Runtime: 24 ms, faster than 20.31% of Rust online submissions for Count the Number of Consistent Strings.
/// Memory Usage: 2.9 MB, less than 6.25% of Rust online submissions for Count the Number of Consistent Strings.
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let filtered: Vec<char> = Self::filter(allowed);
        let mut res = words.len();
        for word in words {
            for one in &filtered {
                if word.contains(*one) {
                    res -= 1;
                    break;
                }
            }
        }
        res as i32
    }

    fn filter(allowed: String) -> Vec<char> {
        let mut char_table: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
        for one in allowed.chars() {
            let index: usize = one as usize - 97;
            char_table.remove(index);
            char_table.insert(index, ' ');
        }
        return char_table;
    }
}
