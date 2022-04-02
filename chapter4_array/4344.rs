use std::io;
use std::fmt::Write;

fn main(){
    let mut cin = String::new();
    io::stdin().read_line(&mut cin).unwrap();
    let c = cin.trim().parse::<usize>().unwrap();
    let mut output = String::new();
    for _ in 0..c {
        write!(output, "{:.3}%\n", get_avg() * 100.0).unwrap();
    }
    println!("{}", output);
}

fn get_avg() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let scorelist: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap()).collect();
    let mut sum = 0;
    let mut cnt = 0;
    for i in 1..scorelist.len() {
        sum += scorelist[i];
    }
    for i in 1..scorelist.len() {
        if (sum as f64 / scorelist[0] as f64) < scorelist[i] as f64 {
            cnt += 1;
        }
    }

    cnt as f64 / scorelist[0] as f64
    
}