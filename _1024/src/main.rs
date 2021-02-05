fn main() {
    let res = Solution::video_stitching(vec![vec![0, 2], vec![4, 6], vec![8, 10], vec![1, 9], vec![1, 5], vec![5, 9]], 10);
    println!("{}", res);
}

struct Solution;
not yet!
impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
        let mut resCnt = 1;
        let mut table: Vec<&Vec<i32>> = Vec::new();

            let res = Self::findList(vec![0, 2], &clips);
            for check in &res {
                if check[1] >= t {
                    return resCnt;
                }
            }
            table = res;
            resCnt+=1;
        return 1

    }


    fn findList(clip: Vec<i32>, clips: &Vec<Vec<i32>>) -> Vec<&Vec<i32>> {
        let mut res: Vec<&Vec<i32>> = vec![];

        for item in clips {
            if item[0] <= clip[1] && item[0] > clip[0] {
                res.push(item);
            }
        }
        return res;
    }
}