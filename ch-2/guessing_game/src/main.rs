use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Go ahead, guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fails");

    println!("You guessed {guess}");
    println!("That's right, {}!", guess);
    println!("...but secret number was {secret_number}")
}
