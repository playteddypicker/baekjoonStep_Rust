use std::io;
use std::cmp;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let nx: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap()).collect();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut elements: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap()).collect();
    elements.push(0);

    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut sum = elements[start];
    let mut length: usize = nx[0] + 1; 

    while end < nx[0] {
        match (sum).cmp(&nx[1]){
            cmp::Ordering::Less => {
                end += 1;
                sum += elements[end];
            },
            cmp::Ordering::Greater | cmp::Ordering::Equal => {
                length = cmp::min(length, end - start + 1);
                sum -= elements[start];
                start += 1;
            },    
        }
    }

    print!("{}", 
        if length == nx[0] + 1 {
            0
        }else{
            length
        }
    );

    Ok(())
}
