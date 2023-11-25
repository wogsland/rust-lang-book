fn main() {
    let fahrenheit = 212;
    println!("Celsius is {}", fahrenheit_to_celcius(fahrenheit));
}

fn fahrenheit_to_celcius(f: i32) -> i32 {
    (f - 32) * 5 / 9
}
