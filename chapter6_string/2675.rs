use std::io;
use std::fmt::Write;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<usize>().unwrap();

    for _ in 0..t{
        let mut output = String::new();
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();

        let n: Vec<&str> = input2.split_whitespace().collect();
        
        for i in n[1].chars() {
            for _ in 0..n[0].trim().parse::<usize>().unwrap(){
                write!(output, "{}", i).unwrap();
            }
        }
        print!("{}\n", output);
    }
}
