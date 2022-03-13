use std::io;

fn main(){
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let num1 = input1.trim().parse::<i32>().unwrap();
    let num2 = input2.trim().parse::<i32>().unwrap();

    println!("{}", num1 * (num2 % 10));
    println!("{}", num1 * ((num2 / 10) % 10));
    println!("{}", num1 * (num2 / 100));
    println!("{}", num1 * num2);
}
