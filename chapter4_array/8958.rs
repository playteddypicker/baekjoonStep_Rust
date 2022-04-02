use std::io;
use std::fmt::Write;

fn main(){
    let mut ninput = String::new();
    io::stdin().read_line(&mut ninput).unwrap();
    let n = ninput.trim().parse::<usize>().unwrap();

    let mut output = String::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        writeln!(output, "{}", get_score(&input)).unwrap();
    }

    print!("{}", output);
}

fn get_score(ox: &str) -> usize {
    let mut i = 0;
    let mut result = 0;
    for (_, &item) in ox.as_bytes().iter().enumerate() {
        if item == b'O' {
            i += 1;
            result += i;    
        }else {
            i = 0;
        }
    }
    result
}