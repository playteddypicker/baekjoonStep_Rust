use std::io;
use std::fmt::Write;

fn nck(n: usize, k: usize) -> usize {
    if n == k {
        1
    }else if k == 1 {
        n
    }else{
        (n-k+1..=n).fold(1, |res, x| res * x) / (1..=k).fold(1, |res, k| res * k)
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_line(&mut input)?;
    for _ in 0..input.trim().parse::<usize>().unwrap() {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let nc: Vec<usize> = input.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap()).collect();
        writeln!(output, "{}", 
            nck(nc[1], 
                if nc[1] - nc[0] < nc[0]{
                    nc[1] - nc[0]
                }else{
                    nc[0]
                }
            )
        ).unwrap();
    }

    print!("{}", output);

    Ok(())
}
