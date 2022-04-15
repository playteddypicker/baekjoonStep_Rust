use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let scicomlove = String::from("SciComLove");
    
    println!("{}{}", &scicomlove[n%10..10], &scicomlove[0..n%10]);
}
