use std::collections::HashSet;

struct Solution {}

fn main() {
    let mut input = Vec::new();
    input.push(vec!["A".to_string(), "B".to_string()]);
    input.push(vec!["C".to_string(), "D".to_string()]);
    input.push(vec!["B".to_string(), "C".to_string()]);

    let res = Solution::dest_city(input);
    println!("{}", res);
}


impl Solution {
    pub fn dest_city(mut paths: Vec<Vec<String>>) -> String {
        let mut set: HashSet<String> = paths.iter_mut().map(|v| v.remove(0)).collect();

        for mut v in paths {
            if !set.contains(&v[0]) {
                return v.remove(0);
            }
        }
        String::new()
    }
}