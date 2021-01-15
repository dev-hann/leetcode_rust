fn main() {
    let res = Solution::to_goat_latin("I speak Goat Latin".to_string());
    println!("{}", res)
}

struct Solution;

// Clean code?
impl Solution {

    pub fn to_goat_latin(s: String) -> String {
        let _vowels = vec!['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

        let mut res: Vec<String> = Vec::new();
        for (index, value) in s.split_whitespace().enumerate() {
            let mut _res_word = String::new();
            let first_char = value.chars().next().unwrap();
            match _vowels.binary_search(&first_char) {
                Ok(_)=>_res_word.push_str(value),
                Err(_)=>{
                    _res_word.push_str(&value[1..]);
                    _res_word.push_str(&value[0..1]);
                }
            }
            _res_word.push_str("ma");
            for _i in 0..index+ 1 {
              _res_word.push('a');
            }
            res.push(_res_word);
        }
        return res.join(" ");
    }
}


// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Goat Latin.
// Memory Usage: 2.1 MB, less than 16.67% of Rust online submissions for Goat Latin.
/*impl Solution {
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
        for (index,s) in res.iter().enumerate() {
            if Self::is_begin_vowel(s) {
                let mut k = s.to_string()+"ma";
                for i in 0..index+1{
                    k=k+"a";
                }
                if index<res.len()-1 {
                    k=k+" ";
                }
                _r.push_str(k.borrow());
                continue;
            }
            let mut a: Vec<char> = s.chars().collect();
            a.push(a[0]);
            a.remove(0);

            let mut x  = a.iter().collect::<String>()+"ma";
            for i in 0..index+1{
                x=x+"a";
            }
            if index<res.len()-1 {
                x=x+" ";
            }
           _r.push_str(x.borrow());
        }

        return _r;
    }
}*/
