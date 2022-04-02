use std::io::{stdin};

fn main(){
    
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    
    for i in 1..n+1{
        let mut input2 = String::new(); 
        stdin().read_line(&mut input2).unwrap();
        let values: Vec<&str> = input2.split_whitespace().collect();
        let a = values[0].trim().parse::<i32>().unwrap();
        let b = values[1].trim().parse::<i32>().unwrap();
        print!("Case #{}: {}\n", i, a + b)
    }
}
