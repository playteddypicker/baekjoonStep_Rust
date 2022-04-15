use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    let t = usize::pow(10, n);

    for i in 1..t {
        println!("{}, {}",i, i.to_string().len());

    }
}
