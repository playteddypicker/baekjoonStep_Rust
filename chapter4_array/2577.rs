use std::io;

fn main() {
  let mut numbers = [0; 10];
  let mut values = [0; 3];

  for i in 0..3{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let v:Vec<i32> = s
      .as_mut_str()
      .split_whitespace()
      .map(|s| s.parse().unwrap())
      .collect();
     values[i] = v[0];
  }
  let cal = values[0] * values[1] * values[2];
  let forstr = cal.to_string().trim().to_uppercase();

  for s in forstr.as_bytes(){
    let save = *s as usize - '0' as usize;
    numbers[save] += 1;
  }
  for rs in &numbers{
    println!("{}", rs);
  }
}

