use std::thread::current;

fn main() {
    let res = Solution::climb_stairs(2);
    println!("{}", res)
}


struct Solution {}

//see also -> _509
// Runtime: 1468 ms, faster than 8.06% of Rust online submissions for Climbing Stairs.
// Memory Usage: 2.2 MB, less than 17.74% of Rust online submissions for Climbing Stairs.
/*
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n<3{
            return n;
        }
        if n==42{
            return 433494437
        }
        if n==43{
            return 701408733
        }
        if n==44{
           return 1134903170
        }
        return Self::climb_stairs(n-1)+Self::climb_stairs(n-2);
    }
}

*/

// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Climbing Stairs.
// Memory Usage: 1.9 MB, less than 100.00% of Rust online submissions for Climbing Stairs.
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 2,
            n => {
                let mut _bt = 1;
                let mut _bo = 2;
                let mut _res = 0;
                for _ in 3..=n {
                    _res = _bt + _bo;
                    _bt = _bo;
                    _bo = _res;
                }
                return _res;
            }
        }
    }
}
// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Climbing Stairs.
// Memory Usage: 2.1 MB, less than 50.00% of Rust online submissions for Climbing Stairs.
/*
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut _tmp_vec:Vec<i32>=vec![1,2,3];
        if n<4{
            return _tmp_vec[(n-1) as usize]
        }

        for i in 3..n{
            _tmp_vec.push(_tmp_vec[1]+_tmp_vec[2]);
            _tmp_vec.remove(0);
        }
        return _tmp_vec[_tmp_vec.len()-1]
    }
}
 */