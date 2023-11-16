use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Go ahead, guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fails");
        let guess: u32 = match guess.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };
        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("...but secret number is bigger"),
            Ordering::Greater => println!("...but secret number is smaller"),
            Ordering::Equal => {
                println!("Bingo! The secret number was {secret_number}.");
                break;
            },
        }
    }
}
