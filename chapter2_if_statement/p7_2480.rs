use std::io;
use std::cmp;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<&str> = input.split_whitespace().collect();
    let a = match values[0].parse::<i32>(){
        Ok(i) => i,
        Err(_) => -1
    };
    let b = match values[1].parse::<i32>(){
        Ok(i) => i,
        Err(_) => -1
    };
    let c = match values[2].parse::<i32>(){
        Ok(i) => i,
        Err(_) => -1
    };

    let result = if a == b && b == c {
        10000 + 1000 * a
    }else if a != b && b != c && c != a {
        cmp::max(cmp::max(a, b), c) * 100
    }else{
        if a == b {
            1000 + 100 * a
        }else if b == c{
            1000 + 100 * b
        }else {
            1000 + 100 * c
        }
    };

    println!("{}", result);

}
