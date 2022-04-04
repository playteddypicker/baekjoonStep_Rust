use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap()).collect();

    println!("{}", (n[0] * n[0] + n[1] * n[1] + n[2] * n[2] + n[3] * n[3] + n[4] * n[4]) % 10);
}
