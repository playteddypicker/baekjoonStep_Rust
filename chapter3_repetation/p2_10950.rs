use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();

    for i in (1..n+1){
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let values: Vec<&str> = input.split_whitespace().collect();
        let a = values[0].parse::<i32>().unwrap();
        let b = values[1].parse::<i32>().unwrap();
        println!("{}", a + b);
    }
}
