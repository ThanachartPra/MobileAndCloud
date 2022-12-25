use std::io::{self, Write};
fn main() {
    print!("Enter a number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x: i32 = input.trim().parse().expect("Failed to parse input");
    for i in 0..x{
        for j in 1..x*2{   
            if (j >= x-i)&&(j <= x+i){
                print!("* ");
            }
            else{
                print!("- ");
            }
        }
        let count = x-i;
        println!("{count}");
    }
}