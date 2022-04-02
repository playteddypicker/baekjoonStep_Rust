use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();

    let _n = input.trim().parse::<i32>().unwrap();
    let numlist: Vec<i32> = input2.split_whitespace()
        .map(|s| s.parse().unwrap()).collect();


    let mut max = -1000000;
    let mut min = 1000000;
    for i in numlist{
        max = if i >= max {
            i
        }else {
            max
        };

        min = if i <= min {
            i
        }else {
            min
        };
    }

    println!("{} {}", min, max);
}
