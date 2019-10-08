use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    println!("Hello Hacktoberfest");
    println!("Quantos termos de fibonacci devem ser calculados?");
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let termos = input.trim().parse::<i32>().expect("Isso não é um numero inteiro!");
    show_fibonacci(termos);
}

fn show_fibonacci(n: i32) {
    let mut f = (1,1);
    for step in 0..n {
        if step > 1 {
            f = (f.1,f.0 + f.1);
        }
        println!("{}", f.1);
    }
}
