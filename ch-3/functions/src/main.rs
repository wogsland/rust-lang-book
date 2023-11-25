fn main() {
    println!("Hello, world!");
    another_function(5, '0');

    let y = {
        let x = 3;
        x + 1
    };
    println!("y is {}.", y);

    let x = five();
    println!("x is {}.", x);
}

fn another_function(x: i32, y: char) {
    println!("This, ladies & gentlemen, is another function...");
    println!("and x squared is {}...", x*x);
    println!("and y is {}.", y);
}

fn five() -> i32 {
    5
}
