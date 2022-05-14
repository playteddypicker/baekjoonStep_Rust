use std::io;

fn main() -> io::Result<()>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>().unwrap();

    print!("{}", tail_factorial(n, 1));
    Ok(())
}

fn tail_factorial(n: usize, total: usize) -> usize {
    if n == 1 || n == 0 {
        return total;
    }
    tail_factorial(n - 1, n * total)
}
