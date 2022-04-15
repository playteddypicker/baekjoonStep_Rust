use std::io;
use std::fmt::Write;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    let list = vec!["0", "1", "10", "11", "100", "101", "110", "111", "1000", "1001", "1010"];

    let mut output = String::new();
    for i in 1..n+1{
        write!(output, "{}", list[i]).unwrap();
    }

    println!("{}", output);
}
