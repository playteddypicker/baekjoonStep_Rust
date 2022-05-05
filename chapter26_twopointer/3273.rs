use std::io;
use std::cmp::Ordering;

fn read_integer() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap()
}

fn main() -> io::Result<()> {
    let n = read_integer();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut elements: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap()).collect();

    let x = read_integer();

    elements.sort();

    let mut start = 0;
    let mut end = n - 1;
    let mut cnt = 0;

    while start < end {
        println!("start: {}, end: {}", start, end);
        match (elements[start] + elements[end]).cmp(&x){
            Ordering::Less => start += 1,
            Ordering::Greater => end -= 1,
            Ordering::Equal => {
                cnt += 1;
                start += 1;
            }
        }
    }

    println!("{}", cnt);

    Ok(())
}
