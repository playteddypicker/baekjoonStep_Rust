use std::io;

fn main(){
    let mut max = 0;
    let mut index = 0;
    for i in 1..10{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n = input.trim().parse::<i32>().unwrap();
        if n >= max {
            max = n;
            index = i;
        }
    }

    print!("{}\n{}", max, index);
}