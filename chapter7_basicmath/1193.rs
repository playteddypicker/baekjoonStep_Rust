use std::io;

fn main(){
    //분자 : 1, 1, 2, 3, 2, 1, 1, 2, 3, 4, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 6,
    //분모 : 1, 2, 1, 1, 2, 3, 4, 3, 2, 1, 1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1,
    //순서 : 1, 2, 3, 4, 5, 6, 7, 8, 9, 10...

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();
    
    let mut a = 1;
    loop {
        if a * (a+1) / 2 >= n {
            break;
        }
        a += 1;
    }
    if a % 2 == 0 {
        println!("{}/{}", a + n - a * (a+1) / 2, 1 - n + a * (a+1)/ 2);
    }else{
        println!("{}/{}", 1 - n + a * (a+1)/ 2, a + n - a * (a+1) / 2);
    }
}
