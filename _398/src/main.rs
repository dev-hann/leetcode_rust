use rand::prelude::*;

fn main() {
    let nums = vec![1, 2, 3, 3, 3];
    let solution = Solution::new(nums);
    let res = solution.pick(3);
    println!("{}", res);
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */


/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */

// Runtime: 16 ms, faster than 100.00% of Rust online submissions for Random Pick Index.
// Memory Usage: 4.5 MB, less than 71.43% of Rust online submissions for Random Pick Index.
struct Solution {
  nums:Vec<i32>
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        return Solution { nums };
    }

    fn pick(&self, target: i32) -> i32 {
        let mut res:Vec<usize>=Vec::new();

        for (index,value) in self.nums.iter().enumerate(){
            if *value!=target{
                continue;
            }
            res.push(index)
        }

        let mut rng=rand::thread_rng();
        return *(res.choose(&mut rng).unwrap()) as i32;
    }
}
