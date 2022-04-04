use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let ascending = String::from("1 2 3 4 5 6 7 8");
    let descending = String::from("8 7 6 5 4 3 2 1");

    if input[0..15] == ascending[0..15] {
        println!("ascending");
    }else if input[0..15] == descending[0..15] {
        println!("descending");
    }else {
        println!("mixed");
    }
}
