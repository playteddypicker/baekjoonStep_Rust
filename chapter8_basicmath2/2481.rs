use std::io;

fn main(){
    let m = read_integer();
    let n = read_integer();

    let mut sum = 0;
    let mut min = 0;

    for i in m..=n {
        if is_prime(i) {
            if sum == 0 {
                min = i;
            }
            sum += i;
        }
    }

    if sum == 0 {
        println!("-1");
    }else {
        println!("{}\n{}", sum, min);
    }
}

fn read_integer() -> isize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<isize>().unwrap()
}

fn is_prime(x: isize) -> bool {
    if x == 1 {
        return false;
    }

    for i in 2..=(x as f64).sqrt() as usize {
        if x % i as isize == 0 {
            return false;
        }
    }
    
    true
}
