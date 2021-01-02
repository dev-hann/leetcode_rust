fn main() {
    let res = Solution::to_lower_case("PiTAs".to_string());
    println!("{}", res);
}

struct Solution {}

impl Solution {

    pub fn to_lower_case(str: String) -> String {
       // return str.to_lowercase();
           let mut res: Vec<char> = vec![];
           for c in str.chars() {
               if 'A' <= c && c <= 'Z' {
                   res.push((c as u8 ^0x20)as char);
               }else{
                   res.push(c)
               }
           }
           return res.iter().collect();
    }

    //Best Solution
    /*
    pub fn to_lower_case(str: String) -> String {
        return  str.chars().map(|c| {
            if 'A' <= c && c <= 'Z' {
                ((c as u8) - b'A' + b'a') as char
            } else {
                c
            }
        }).collect();
    }
    */

}
