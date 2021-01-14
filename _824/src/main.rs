use std::borrow::Borrow;

fn main() {
    let res = Solution::to_goat_latin("I speak Goat Latin".to_string());
    println!("{}", res)
}

struct Solution {}

//Not yet solved!
impl Solution {
    fn is_begin_vowel(str: &str) -> bool {
        let _vowel: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
        let a = str.chars().collect::<Vec<char>>()[0].to_ascii_lowercase();
        for v in _vowel.iter() {
            if a == *v {
                return true;
            }
        }
        return false;
    }

    pub fn to_goat_latin(s: String) -> String {
        let res: Vec<&str> = s.split(" ").collect();
        let mut _r: String = "".to_string();
        for s in res.iter() {
            if Self::is_begin_vowel(s) {
            //    print!("{:?}", s);
                let k = s.to_string()+" ";
                _r.push_str(k.borrow());
                continue;
            }
            let mut a: Vec<char> = s.chars().collect();
            a.push(a[0]);
            a.remove(0);

            //print!("{:?}", (a.iter().collect::<String>()));
           _r.push_str(&a.iter().collect::<String>());

        }
        println!("{}", _r);

        return res.join(" ");
    }
}