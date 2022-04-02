use std::io::{stdin};
use std::fmt::Write;

fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let mut output = String::new();
    for i in 1..n+1{
        let mut output2 = String::new();
        for _ in (1..n-i+1).rev(){
            write!(output2, " ").unwrap();
        }
        for _ in 1..i+1{
            write!(output2, "*").unwrap();
        }
        writeln!(output, "{}", output2).unwrap();
    }
    // 마지막에 한 번만 호출
    print!("{}", output);
}
