use std::io;

fn main(){
    let mut cin = String::new();
    let mut input = String::new();
    io::stdin().read_line(&mut cin).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    
    let mut sum = 0;
    for i in 0..cin.trim().parse::<usize>().unwrap() {
        sum += input.as_bytes()[i] as usize - 48;
    }

    print!("{}", sum);
    
}