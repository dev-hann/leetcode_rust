fn main() {
    let res: bool = Solution::repeated_substring_pattern("ababab".to_string());

    println!("{}", res)
}


struct Solution;

// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Repeated Substring Pattern.
// Memory Usage: 2 MB, less than 75.00% of Rust online submissions for Repeated Substring Pattern.
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let len = s.len();
        for i in (1..=len / 2).rev() {
            ///more faster
            if len % i != 0 {
                continue;
            }
            //than this
            ///if len%i==0 {
            let mut _tmp_str = String::new();
            let substring = (&s[..i]).clone();
            if s.eq(&substring.repeat(len / i)) {
                return true;
            }
        }
        return false;
    }
}

// Runtime: 12 ms, faster than 83.33% of Rust online submissions for Repeated Substring Pattern.
// Memory Usage: 2.1 MB, less than 50.00% of Rust online submissions for Repeated Substring Pattern.
/*impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {

        let len = s.len();

        let mut split_cnt=2;
        while split_cnt<=len {
            if len%split_cnt==0{
                let (one,other)= s.split_at(len/split_cnt);
                let res = other.replace(one,"");
                if res==""{
                    return  true
                }
            }
            split_cnt+=1;
        }
        return false
    }
}*/