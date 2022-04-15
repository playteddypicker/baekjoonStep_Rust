use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let abc: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap()).collect();
    // a + b*n <= c*n이 최초인 지점
    // n >= a / (c-b) + 1
    if abc[1] >= abc[2] {
        println!("{}", -1);
    }else {
        println!("{}", abc[0] / (abc[2] - abc[1]) + 1);
    }
}
