use std::io;
use std::fmt::Write;

fn main() -> io::Result<()> {    
    let mut fastoutput = String::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let m = input.trim().parse::<usize>().unwrap();
        
        if m == 0 {
            break;
        }

        let mut cnt = 0;
        for x in prime_list(2 * m){
            if x <= m || x == 0 || x == 1{
                continue;
            }else{
                cnt += 1;
            }
        }
        writeln!(fastoutput, "{}", cnt).unwrap();
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
