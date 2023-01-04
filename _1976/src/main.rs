#[derive(Debug)]
struct Solution;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut max = -1;
        let mut res = -1;
        for c in s.chars() {
            match c.to_digit(10) {
                Some(a) => {
                    let i = a as i32;
                    if max < i {
                        res = max;
                        max = i;
                    }
                    if max > i && res < i {
                        res = i;
                    }
                }
                None => continue,
            }
        }
        return res;
    }
}

// 1796. Second Largest Digit in a String
fn main() {
    let s = String::from("sjhtz8344");
    let res = Solution::second_highest(s);
    println!("{:}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set1() {
        let s = String::from("sjhtz8344");
        let res = Solution::second_highest(s);
        assert_eq!(res, 4);
    }
    #[test]
    fn set2() {
        let s = String::from("dfa12321afd");
        let res = Solution::second_highest(s);
        assert_eq!(res, 2);
    }
    #[test]
    fn set3() {
        let s = String::from("abc1111");
        let res = Solution::second_highest(s);
        assert_eq!(res, -1);
    }
}
