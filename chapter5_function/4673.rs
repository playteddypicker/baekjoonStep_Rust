fn main() {
    let mut numlist: [usize; 20000] = [0; 20000];
    numlist[0] = 1;
    for i in 1..10001 {
        numlist[selfnum_generator(i as usize)] = 1;
    }

    for i in 1..10001 {
        if numlist[i as usize - 1] != 1 {
            println!("{}", i - 1);
        }
    }
}

fn selfnum_generator(mut i: usize) -> usize {
    let mut res: usize = i;
    while i != 0 {
        //i가 1의자리가 될 때까지
        res += i % 10; //1의자리부터
        i /= 10;
    }
    res
}

