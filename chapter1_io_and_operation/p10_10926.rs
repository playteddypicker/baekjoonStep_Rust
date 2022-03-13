use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut s = input.to_string();
    s.pop();
    println!("{}??!", &s);
}
