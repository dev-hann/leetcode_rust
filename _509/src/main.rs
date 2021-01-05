fn main() {
    let res = Solution::fib(3);
    println!("{}", res);
}


struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n==0 || n==1{
            return n
        }
        let mut fib_list = vec![0, 1];
        for _i in 2..n+1 {
            fib_list.push(fib_list[0]+fib_list[1]);
            fib_list.remove(0);
        }
        return fib_list[fib_list.len()-1];
    }
}