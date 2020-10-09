use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game");
    let mut generator = rand::thread_rng();
    let secret_number = generator.gen_range(1, 101);

    loop {
        println!("Please enter your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read, yikes.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
