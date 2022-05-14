use std::io;

fn main() -> io::Result<()>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<isize>().unwrap();

    print!("{}", tail_fibo(n, 0));
    Ok(())
}

fn tail_fibo(n: isize, total: isize) -> isize {
    if n < 2 {
        return total + n
    }
    tail_fibo(n - 1, total) + tail_fibo(n - 2, total)
}
