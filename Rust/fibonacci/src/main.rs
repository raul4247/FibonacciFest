use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    println!("Hello Hacktoberfest!");
    println!("Quantos termos de fibonacci devem ser calculados?");
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let termos = match input.trim().parse::<i32>() {
        Ok(ok) => ok,
        Err(_) => 0,
    };
    show_fibonacci(termos);
}

fn show_fibonacci(n: i32) {
    let mut f = (0,1);
    for _ in 0..n {
        println!("{}", f.1);
        f = (f.1,f.0 + f.1);
    }
}
