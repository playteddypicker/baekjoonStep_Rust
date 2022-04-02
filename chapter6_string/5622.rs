use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    //하드코딩 박는게 빠를듯
    let numlist = [
        3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 10, 10, 10, 10,
    ];
    input = input.trim().to_uppercase();

    let mut res = 0;
    for c in input.as_bytes() {
        res += numlist[*c as usize - 'A' as usize];
    }

    println!("{}", res);
}
