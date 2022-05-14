use std::io;
use std::fmt::Write;

fn main() -> io::Result<()>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut n = input.trim().parse::<usize>().unwrap();

    let mut fastwrite = String::new();

    for i in 2..=(n as f64).sqrt() as usize {
        while n % i == 0 {
            writeln!(fastwrite, "{}", i).unwrap();
            n /= i;
        }
    }

    if n != 1 {
        write!(fastwrite, "{}", n).unwrap();
    }

    print!("{}", fastwrite);
    Ok(())
}
