use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let asdf = input.trim().parse::<i32>().unwrap();

    if ((asdf % 4 == 0) && (asdf % 100 != 0)) || (asdf % 400 == 0) {
        println!("1");
    }else{
        println!("0");
    }
}
