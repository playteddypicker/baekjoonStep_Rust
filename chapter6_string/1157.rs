use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut alphabets = [0; 26];

    for i in input.trim().to_lowercase().as_bytes().iter(){
        alphabets[*i as usize - 'a' as usize] += 1;
    }

    let max_value = alphabets.iter().max();
    match max_value {
        Some(max) => {
            let mut n = 0;
            for i in alphabets.iter(){
                if i == max {
                    n += 1;
                }
            }
            if n < 2 {
                println!("{}", ('A' as u8 + alphabets.iter().position(|e| e == max).unwrap() as u8) as char);
            }else {
                println!("?");
            }
        },
        None => println!("vector is empty"),
    }

}
