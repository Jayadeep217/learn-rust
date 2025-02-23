use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("!!!Guess a number!!!");

    let secret_number = rand::rng().random_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guess : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
