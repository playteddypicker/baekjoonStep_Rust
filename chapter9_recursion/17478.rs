use std::io;
use std::fmt::Write;
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>().unwrap();

    println!("어느 한 컴퓨터공학과 학생이 유명한 교수님을 찾아가 물었다.");
    let mut output = String::new();
    whatisrecursion(n, 0, &mut output);
    print!("{}", output);
    println!("라고 답변하였지.");

    Ok(())
}

fn whatisrecursion(n: usize, a: usize, output: &mut String){
    writeln!(output, 
        "{}\"재귀함수가 뭔가요?\"", 
        "_".repeat(4 * a)).unwrap();

    if a == n {
        writeln!(output, 
            "{}\"재귀함수는 자기 자신을 호출하는 함수라네\"", 
            "_".repeat(4 * a)).unwrap();
        return;
    }

    writeln!(output, 
        "{}\"잘 들어보게. 옛날옛날 한 산 꼭대기에 이세상 모든 지식을 통달한 선인이 있었어.", 
        "_".repeat(4 * a)).unwrap();
    writeln!(output, 
        "{}마을 사람들은 모두 그 선인에게 수많은 질문을 했고, 모두 지혜롭게 대답해 주었지.", 
        "_".repeat(4 * a)).unwrap();
    writeln!(output, 
        "{}그의 답은 대부분 옳았다고 하네. 그런데 어느 날, 그 선인에게 한 선비가 찾아와서 물었어.\"", 
        "_".repeat(4 * a)).unwrap();

    whatisrecursion(n, a + 1, output);

    writeln!(output, 
        "{}라고 답변하였지.", 
        "_".repeat(4 * a + 4)).unwrap();
}
