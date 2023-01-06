struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }
        if dividend == divisor {
            return 1;
        }
        let div: i32 = divisor.abs();
        let mut target: i32 = dividend.abs();
        let mut res: i32 = 0;
        while target > 0 {
            target = target - div;
            if target >= 0 {
                res = res + 1;
            }
        }
        if dividend < 0 && divisor < 0 || dividend > 0 && divisor > 0 {
            return res;
        }
        return -res;
    }
}

fn main() {
    let res = Solution::divide(10, 3);
    println!("{}", res)
}
