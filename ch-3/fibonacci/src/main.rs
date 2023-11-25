fn main() {
    let nth = 5;
    println!("The {}th Fibonacci number is {}",  nth, fibonacci(nth));
}

fn fibonacci(n: i32) -> i32 {
    let mut current = 1;
    let mut prior = 0;
    let mut count = 1;
    loop {
        if count == n {
            break current;
        }
        let new_current = current + prior;
        prior = current;
        current = new_current;
        count += 1;
    }
}
