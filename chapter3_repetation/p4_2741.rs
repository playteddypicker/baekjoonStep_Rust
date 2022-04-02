use std::io::{stdin};
use std::fmt::Write;

fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let mut output = String::new();
    for i in (1..n+1).rev(){
        writeln!(output, "{}", i).unwrap();
    }
    // 마지막에 한 번만 호출
    print!("{}", output);
}
