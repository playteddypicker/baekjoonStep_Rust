use std::io;
use std::fmt::Write;

fn main() -> io::Result<()>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let nm: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap()).collect();

    let mut output = String::new();
    let mut check = vec![0; 10];
    let mut toprint_arr = Vec::new();
    recursive_print(nm[0], nm[1], 1, &mut output, &mut toprint_arr, &mut check);

    print!("{}", output);
    Ok(())
}

fn recursive_print(n: usize, m: usize, st: usize, 
    output: &mut String, arr: &mut Vec<usize>, check: &mut Vec<usize>) {
    println!("{:?}", arr);

    if arr.len() == m { 
        writeln!(output, "{}", 
            arr.into_iter()
            .map(|i| i.to_string() + " ").collect::<String>()
        ).unwrap();
        return;
    }

    for i in st..=n {
        if let _ = match arr.iter().find(|&&x| x != i){
            Some(_) => true,
            None => false
        } {
            arr.push(i);
            check[i] = 1;
            recursive_print(n, m, st+1, output, arr, check);
            check[i] = 0;
            arr.pop();
        }
    }
}
