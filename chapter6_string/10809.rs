use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let a = input.as_bytes()[0];
    print!("{}", a as usize);
}