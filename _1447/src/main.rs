fn main() {
    let res = Solution::simplified_fractions(10);
    println!("{:?}", res)
}

struct Solution;

//GCD Problem Not Yet!!
impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { Self::gcd(b, a % b)}
    }
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        let mut res = vec![];
        for i in 2..n+1{
            for j in 1..i {
                if Self::gcd(i, j) == 1 {
                    res.push(j.to_string() + "/" + &i.to_string());
                }
            }
        }
        res
    }
}





// Runtime: 48 ms, faster than 45.45% of Rust online submissions for Simplified Fractions.
// Memory Usage: 2.3 MB, less than 36.36% of Rust online submissions for Simplified Fractions.
// first Ans

/*
impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        if n == 1 {
            return vec![];
        }
        let mut res: Vec<String> = vec![];
        for i in 2..=n {
            for j in 1..i {
                if j == 1 {
                   res.push(format!("{}/{}",j,i))
                } else {
                    if i % j != 0 {
                        let mut _check:bool = true;
                        for nums in 2..=50{
                          if j%nums==0 && i%nums==0 {
                              _check=false;
                              break;
                          }
                      }
                        if _check{
                            res.push(format!("{}/{}",j,i))
                        }
                    }
                }
            }
        }
        return res;
    }
}*/