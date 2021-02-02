use std::collections::HashMap;
use rand::seq::SliceRandom;

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

not yet!!

// Runtime: 64 ms, faster than 14.29% of Rust online submissions for Random Pick Index.
// Memory Usage: 5.7 MB, less than 14.29% of Rust online submissions for Random Pick Index.
struct Solution {
    map: HashMap<i32,Vec<usize>>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut _tmp_hash: HashMap<i32,Vec<usize>> = HashMap::new();
        for (index, value) in nums.iter().enumerate() {
            match _tmp_hash.get_mut(value) {
                None => { _tmp_hash.insert(*value, vec![index]); }
                Some(va) => {va.push(index)}
            }
        }
        return Solution { map: _tmp_hash };
    }

    fn pick(&self, target: i32) -> i32 {
        let mut rng = rand::thread_rng();
        let res = self.map.get(&target).unwrap();
        let x = res.choose(&mut rng).unwrap();
        return *x as i32
    }
}
