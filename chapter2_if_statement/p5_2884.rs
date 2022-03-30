use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<&str> = input.split_whitespace().collect();
    let hour = match values[0].parse::<i32>(){
        Ok(i) => i,
        Err(_e) => {
            -1
        }
    };
    let min = match values[1].parse::<i32>(){
        Ok(i) => i,
        Err(_e) => {
            -1
        }
    };

    let hourresult = if min - 45 < 0 {
        if hour == 0 {
            23
        }else{
            hour - 1
        }
    }else{
        hour
    };

    let minresult = if min - 45 < 0 {
        60 - 45 + min
    }else {
        min - 45
    };
    
    println!("{} {}", hourresult, minresult)

}
