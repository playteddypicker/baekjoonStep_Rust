use std::io;
use std::cmp;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>().unwrap();

    if n == 1 {
        print!("0");
        return Ok(())
    }

    let mut elements = prime_list(n);

    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut sum = elements[start];
    let mut cnt = 0; 

    let x = elements.len();
    elements.push(0);

    while end < x {
        match (sum).cmp(&n){
            cmp::Ordering::Less => {
                end += 1;
                sum += elements[end];
            },
            cmp::Ordering::Greater => {
                sum -= elements[start];
                start += 1;
            } cmp::Ordering::Equal => {
                cnt += 1;
                end += 1;
                sum += elements[end];
            },    
        }
    }

    print!("{}", cnt);

    Ok(())
}

fn prime_list(n: usize) -> Vec<usize> {
    let mut primelist: Vec<usize> = (0..=n).collect();

    for i in 2..=n {
        if primelist[i] != 0 {
            for j in 2..=n / i {
                primelist[i * j] = 0;
            }
        }
    }

    let mut resultlist: Vec<usize> = Vec::new();

    for i in 0..=n {
        if primelist[i] != 0 && primelist[i] != 1 {
            resultlist.push(primelist[i]);
        }
    }

    resultlist
}
