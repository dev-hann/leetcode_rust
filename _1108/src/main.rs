struct Solution{}


fn main() {
    let input = String::from("1.1.1.1");
   let res = Solution::defang_i_paddr(input);
    println!("{}",res);
}

impl Solution{
    pub fn defang_i_paddr(address: String) -> String {
        address.replace('.',"[.]")
    }
}