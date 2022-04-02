use std::io;

fn main(){
    let mut input = String::new();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input).unwrap();
    io::stdin().read_line(&mut input2).unwrap();
    let n = input.trim().parse::<i32>().unwrap();
    let scorelist: Vec<usize> = input2.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap()).collect();
    let mut max = 0;
    let mut sum = 0;
    
    for score in scorelist {
        if score >= max {
            max = score;
        }
        sum += score;
    }

    println!("{}", sum as f64 / max as f64 * 100.0 / n as f64);
}