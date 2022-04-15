use std::io;

fn main(){
    // 1 : 0
    // 1 + 1 ~ 1 + 6 : 1 6개
    // 8 ~ 19 : 12개
    // 20 ~ 37 : 18개
    // 1 + 6n ~ 1 + 12n까지 n칸
    //
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u64>().unwrap();

    let mut range: u64 = 1;
    let mut cnt: u64 = 1;
    while n > range {
        range += 6 * cnt;
        cnt += 1;
    }
    println!("{}", cnt);
}
