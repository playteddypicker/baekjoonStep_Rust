use std::io;
use std::fmt::Write;
    
fn main(){
    let mut output = String::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let v: Vec<i32> = input.split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        if v[0] == 0 {
            break;
        }
        
        writeln!(output, "{}", v[0] + v[1]).unwrap();
    }

    println!("{}", output);

}
