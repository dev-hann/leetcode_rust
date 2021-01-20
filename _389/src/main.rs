

fn main() {
    let res = Solution::find_the_difference(String::from("a"), String::from("aa"));
    println!("{}", res)
}

struct Solution;
// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Find the Difference.
// Memory Usage: 2.1 MB, less than 69.57% of Rust online submissions for Find the Difference.
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let t_sum: u8 = t.chars().map(|x|x as u8).sum();
        let s_sum: u8 = s.chars().map(|x|x as u8).sum();
        return char::from(t_sum-s_sum);
    }
}


// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Find the Difference.
// Memory Usage: 2.2 MB, less than 8.70% of Rust online submissions for Find the Difference.
// toooo much memory
/*
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut char_table: HashMap<char, i32> = HashMap::new();
        for c in t.chars() {
            let v = match char_table.get(&c) {
                Some(char) => char,
                None => &0
            };
            char_table.insert(c, *v + 1);
        }
        for c in s.chars() {
            let v = match char_table.get(&c) {
                Some(char) => char,
                None => &0
            };
            char_table.insert(c, *v - 1);
        }
        let mut res = ' ';
        for (c, v) in char_table.iter() {
            if *v == 1 {
                res = *c;
            }
        }

        return res;
    }
}*/