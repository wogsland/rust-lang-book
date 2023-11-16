use std::io;

fn main() {
    println!("Welcome to Guess the Number!");

    println!("Go ahead, guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fails");

    println!("You guessed {guess}");
    println!("That's right, {}!", guess);
}
