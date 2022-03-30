use std::io;

fn main(){
    let mut input = String::new();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input).unwrap();
    io::stdin().read_line(&mut input2).unwrap();

    let values: Vec<&str> = input.split_whitespace().collect();
    let aftertime = match input2.trim().parse::<i32>(){
        Ok(i) => i,
        Err(_e) => {
            -1
        }
    };

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

    let minresult = (aftertime % 60 + min) % 60;
    let hourresult = if (min + aftertime) / 60 + hour == 24 {
        0
    }else {
        (min + aftertime) / 60 + hour
    };

    println!("{} {}", hourresult, minresult)

}
