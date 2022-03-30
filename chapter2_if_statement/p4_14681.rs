use std::io;

fn main(){
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let asdf1 = input1.trim().parse::<i32>().expect("this is not number");
    
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let asdf2 = input2.trim().parse::<i32>().expect("this is not number");

    if asdf1 > 0 && asdf2 > 0 {
        println!("1");
    }else if asdf1 > 0 && asdf2 < 0{
        println!("4");
    }else if asdf1 < 0 && asdf2 > 0{
        println!("2");
    }else {
        println!("3");
    }

}
