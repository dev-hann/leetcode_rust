fn main() {
    let res = Solution::add_digits(38);
    println!("{}", res);
}

// Follow up:
// Could you do it without any loop/recursion in O(1) runtime?
struct Solution;

// first using for loop
// not Yet
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num_vec:Vec<i32>=Vec::new();
        let mut res=num as u32;
        while res.to_string().len()!=1 {
            for c in res.to_string().chars(){
                let r = match c.to_digit(10) {
                    Some(value)=>value,
                    None=>0,
                };
                res=res+r;
            }
            println!("{:?}",res);
        }


        1
    }
}