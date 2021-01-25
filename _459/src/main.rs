fn main() {
    let res: bool = Solution::repeated_substring_pattern("aba".to_string());

    println!("{}", res)
}

struct Solution {}

Not Yet!!
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let len = s.len();

        let mut one = "".to_string();
        for i in 0..len/2 {

            let (one,other) = s.split_at(i);
           if other.replace(one,"")==""{
               return true
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