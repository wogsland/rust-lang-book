use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("Hello, world!");
    crate::garden::hello();
    let plant = Asparagus{};
    println!("I'm growing {:?}!", plant);
}
