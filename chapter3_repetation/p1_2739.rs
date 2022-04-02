use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();

    for num in (1..10){
        println!("{} * {} = {}", n, num, n * num);
    }

}
