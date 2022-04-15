use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    println!("{}{}", &input, &input2);
}
