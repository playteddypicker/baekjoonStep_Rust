use std::io;
use std::cmp::Ordering;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let ab: Vec<&str> = input.split_whitespace().collect();
  let a = ab[0].chars().rev().collect::<String>().parse::<i32>().unwrap();
  let b = ab[1].chars().rev().collect::<String>().parse::<i32>().unwrap();

  match a.cmp(&b){
    Ordering::Less => println!("{}", b),
    Ordering::Greater => println!("{}", a),
    Ordering::Equal => println!("{}", a),
  }
}