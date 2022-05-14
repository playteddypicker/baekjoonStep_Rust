use std::io;
use std::fmt::Write;

fn main() -> io::Result<()> {    
    let mut output = String::new();

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim() == "0" { break; }

        writeln!(output, "{}", 
            if palindrome(input.trim().parse::<usize>().unwrap(), 1) == 1 {
                "yes"
            }else {
                "no"
            }
        ).unwrap();
    }

    print!("{}", output);

    Ok(())
}

fn palindrome(n: usize, digit: usize) -> usize {
    if n / digit % 10 != n / (find_digit(n) / digit) % 10 {
        return 0;
    }

    if digit * digit >= find_digit(n) {
        return 1;
    }

    return palindrome(n, digit * 10);
}

fn find_digit(n: usize) -> usize {
    if n < 10 {
        return 1;
    }

    return 10 * find_digit(n / 10);
}
