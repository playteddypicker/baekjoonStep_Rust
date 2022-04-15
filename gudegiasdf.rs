use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n1 = input.trim().parse::<usize>().unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let n2 = input2.trim().parse::<usize>().unwrap();
    
    println!("{}", n2 * (n2 + 1) / 2);
}
