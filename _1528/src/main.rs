
struct Solution{

}
fn main() {
    let str = String::from("codeleet");
    let indices = vec![4,5,6,7,0,2,1,3];
    let res=Solution::restore_string(str,indices);
    println!("{}",res);
}

impl Solution {
    pub fn restore_string(s:String, indices: Vec<i32>) -> String {
        let length=s.len();
        let mut res = vec![' ';length];
        let mut cnt =0;
        for i in s.chars(){
            res[indices[cnt] as usize]=i;
            cnt+=1;
        }
       res.iter().collect()
    }
}