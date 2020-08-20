

struct Solution{}

fn main() {
    let x:Vec<i32>=vec!(48000,59000,99000,13000,78000,45000,31000,17000,39000,37000,93000,77000,33000,28000,4000,54000,67000,6000,1000,11000);
    print!("{:?}",Solution::average(x));


}

impl Solution{
    pub fn average(salary:Vec<i32>)->f64{
    let mut _temp = salary;
        _temp.sort();
        let res=&_temp[1.._temp.len()-1];
        let sum:i32 = res.iter().sum();
        let result:f64=sum as f64/res.len() as f64;
        result as f64

    }

}