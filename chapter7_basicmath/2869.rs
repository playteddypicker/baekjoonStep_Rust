use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let abv: Vec<u32> = input.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap()).collect();
    print!("{}", (abv[2] - abv[1] - 1) / (abv[0] - abv[1]) + 1);
}
