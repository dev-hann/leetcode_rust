fn main() {
    let res = Solution::to_goat_latin("I speak Goat Latin".to_string());
    println!("{}", res)
}

struct Solution {}
Not yet solved!
impl Solution {
    fn isContainVowel(str: &str) -> bool {
        let _vowel: Vec<&str> = vec!["a", "e", "i", "o", "u"];
        for v in _vowel.iter() {
            if (str.to_lowercase()).contains(v) {
                return true
            }
        }
        return false;
    }

    pub fn to_goat_latin(s: String) -> String {
        let mut res: Vec<&str> = s.split(" ").collect();
        for mut s in res.iter() {
            if Self::isContainVowel(s) {
                s= &&*(s.clone().to_owned() + "ma");
            }
        }


        return res.join(" ");
    }
}