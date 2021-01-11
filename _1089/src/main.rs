fn main() {
    let arr: &mut Vec<i32> = &mut vec![1, 0, 2, 3, 0, 4, 5, 0];
    println!("{:?}", arr);
    Solution::duplicate_zeros(arr);
    println!("{:?}", arr);
}

struct Solution {}
// Runtime: 4 ms, faster than 25.93% of Rust online submissions for Duplicate Zeros.
// Memory Usage: 2.1 MB, less than 62.96% of Rust online submissions for Duplicate Zeros.
/*
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let len = arr.len();
        let mut _clone = vec![];
        for index in 0..len {
            let value = arr[index];
            if value == 0 {
                _clone.push(value)
            }
            _clone.push(value)
        }
        let _res:Vec<i32> = (&_clone[0..len]).to_vec();
        *arr = _res
    }
}

*/
//more clean
// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Duplicate Zeros.
// Memory Usage: 2.3 MB, less than 29.63% of Rust online submissions for Duplicate Zeros.
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let len = arr.len();
        let mut i=0;
        while i<len {
            if arr[i]==0{
                arr.insert(i,0);
                arr.remove(len);
                i+=1;
            }
            i+=1;
        }
    }
}