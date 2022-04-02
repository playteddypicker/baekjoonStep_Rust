use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();
    let mut x: i32;
    let mut nx: i32 = n;
    let mut count: i32 = 0;

    let result = if n != 0 {
        loop{
            if nx / 10 == 0 {
                x = nx * 10 + nx;
                nx = x;
                count = count + 1;
            }else{
                x = nx / 10 + nx % 10;
                nx = (nx % 10) * 10 + x % 10;
                count = count + 1;
            }
            if nx == n { break; }
        }
        count
    }else{
        1
    };

    println!("{}", result);

}
