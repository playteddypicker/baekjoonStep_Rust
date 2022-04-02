use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    
    if n < 100 {
        print!("{}", n);
    }else {
        let mut cnt = 99;
        for i in 100..n+1 {
            if i / 100 + i % 10 == 2 * ((i % 100) / 10) {
                cnt += 1;
            }
        }
        print!("{}", cnt);
    }
}