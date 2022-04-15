use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n1 = input.trim().parse::<usize>().unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let n = input2.trim().parse::<usize>().unwrap();
  
    let mut sum = 0;
    for i in 1..n+1 {
        sum += i * (i+1) / 2;
    }
    println!("{}", sum);
}
