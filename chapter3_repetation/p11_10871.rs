use std::io;
use std::fmt::Write;

fn main(){
    let mut input = String::new();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input).unwrap();
    io::stdin().read_line(&mut input2).unwrap();

    let v1: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap()).collect();

    let numbers: Vec<i32> = input2.split_whitespace()
        .map(|s| s.parse().unwrap()).collect();

    let mut output = String::new();
    for i in numbers {
        if i < v1[1] {
            write!(output, "{} ", i).unwrap();
        }
    }
    print!("{}", output);
}
