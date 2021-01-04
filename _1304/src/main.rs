fn main() {
    let res = Solution::sum_zero(5);
    println!("{:?}", res);
}

struct Solution {}

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut res = vec![];
        if n % 2 == 0 {
            for one in 1..n/2+1 {
                res.push(one);
                res.push(-one);
            }
        } else {
            for one in 1..n/2+1 {
                res.push(one);
                res.push(-one);
            }
            res.push(0);
        }
        return res;
    }
}