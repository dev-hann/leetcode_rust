fn main() {
    let res = Solution::video_stitching(vec![vec![0,1],vec![6,8],vec![0,2],vec![5,6],vec![0,4],vec![0,3],vec![6,7],vec![1,3],vec![4,7],vec![1,4],vec![2,5],vec![2,6],vec![3,4],vec![4,5],vec![5,7],vec![6,9]], 9);
   // let res = Solution::video_stitching(vec![vec![5,7],vec![1,8],vec![0,0],vec![2,3],vec![4,5],vec![0,6],vec![5,10],vec![7,10]], 5);
    println!("{}", res);
}

struct Solution;
not yet!
impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
        if t==0{
            return 0
        }
        let mut resCnt = 1;

        let mut table: Vec<Vec<i32>>=Vec::new();
        for item in clips.clone(){
            if item[0]==0{
                table.push(item);
            }
        }
        for check in &table {
            if check[1] >= t {
                return  resCnt;
            }
        }

        loop{
            let res = Self::findList(table, clips.clone());
            if res.len()==0{
                return -1;
            }
            resCnt += 1;
            for check in &res {
                if check[1] >= t {
                    return resCnt
                }
            }
            table = res;
        }
        return resCnt;

    }


    fn findList(clip: Vec<Vec<i32>>, clips: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        for c in clip{
            for cl in clips.clone(){
                if cl[0]<=c[1] && cl[0]>c[0]{
                   if !res.contains(&cl){
                       res.push(cl);
                   }
                }
            }
        }

        return res;
    }
}