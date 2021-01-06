fn main() {
    let res = Solution::fib(3);
    println!("{}", res);
}


struct Solution {}
//Runtime: 0 ms, faster than 100.00% of Rust online submissions for Fibonacci Number.
// Memory Usage: 2.3 MB, less than 7.89% of Rust online submissions for Fibonacci Number. ->can reduce?
/*impl Solution {
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
}*/

//Runtime: 8 ms, faster than 12.50% of Rust online submissions for Fibonacci Number.
// Memory Usage: 2.1 MB, less than 45.83% of Rust online submissions for Fibonacci Number.
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n<=1{
            return n
        }
      return Self::fib(n-1)+Self::fib(n-2)
    }
}