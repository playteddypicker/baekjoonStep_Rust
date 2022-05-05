use std::io;
use std::cmp::Ordering;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut elements: Vec<isize> = input.split_whitespace()
        .map(|s| s.parse::<isize>().unwrap()).collect();
    elements.sort();

    let mut start: usize = 0;
    let mut end = n - 1;
    let mut min = 3000000000;
    let mut a = 0;
    let mut b = 0;

    while start < end {
        min = if min > (elements[start] + elements[end]).abs() {
            a = start;
            b = end;
            (elements[start] + elements[end]).abs()
        } else {
            min
        };

        match (elements[start] + elements[end]).cmp(&0){
            Ordering::Less => start += 1,
            Ordering::Greater => end -= 1,
            Ordering::Equal => break,
        }
    }

    println!("{} {}", elements[a], elements[b]);

    Ok(())
}
