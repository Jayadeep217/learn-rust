use std::io;

fn main() {
    println!("!!!Guess a number!!!");

    println!("input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to readline!");

    println!("your guess : {}", guess);
}
