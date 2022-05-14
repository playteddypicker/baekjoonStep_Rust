use std::io;
use std::fmt::Write;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mn: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap()).collect();

    let mut fastoutput = String::new();

    for x in prime_list(mn[1]){
        if x < mn[0] || x == 0 || x == 1{
            continue;
        }else{
            writeln!(fastoutput, "{}", x).unwrap();
        }
    }

    println!("{}", fastoutput);

    Ok(())
}

fn prime_list(n: usize) -> Vec<usize> {
    let mut primelist: Vec<usize> = (0..=n).collect();

    for i in 2..=n {
        for j in 2..=n / i {
            primelist[i * j] = 0;
        }
    }

    primelist
}
