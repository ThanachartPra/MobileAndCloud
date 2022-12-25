use std::io::{self, Write};
fn main() {
    print!("Enter a number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let _num: i32 = input.trim().parse().expect("Failed to parse input");
    let prime:[i32;25] = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97];
    let mut now = _num;
    let mut check = true;
    print!("{_num} = ");
    for i in prime.iter() {
        while check{
            if now%i == 0{
                if now == *i{
                    print!("{i}");
                }else{
                    print!("{i}*");
                }
                now = now/i;
            }else{
                check = false;
            }
        }
        check = true;
    }  
}
