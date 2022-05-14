use std::io;
use std::fmt::Write;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>().unwrap();

    let mut output = String::new();
    for i in 0..n {
        let mut temp = "*".repeat(n);
        for j in 0..n {
            recursive_stars(i, j, n, &mut temp);
        }
        write!(output, "\n").unwrap();
    }
    print!("{}", output);

    Ok(())
}

fn recursive_stars(i: usize, j: usize, n: usize, temp: &mut String) {
    if (i / n) % 3 == 1 && (j / n) % 3 == 1 {
        temp.as_bytes()[j] = ' ' as u8;
    }else {
        recursive_stars(i, j, n / 3, temp);
        return;
    }
}
