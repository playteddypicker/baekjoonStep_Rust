use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let _ = input.trim().parse::<usize>().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let nlist: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap()).collect();
    
    let mut cnt = 0;
    for i in nlist {
        if is_prime(i){
            cnt += 1;
        }
    }

    print!("{}\n", cnt);

    Ok(())
}

fn is_prime(x: usize) -> bool {
    if x == 1 {
        return false;
    }

    for i in 2..=(x as f64).sqrt() as usize {
        if x % i == 0 {
            return false;
        }
    }
    
    true
}
