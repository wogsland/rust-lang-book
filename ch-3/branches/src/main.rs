fn main() {
    let johnny = 3;
    if johnny < 5 {
        println!("Destroy, Destroy, Destroy.");
    } else if johnny == 5 {
        println!("Johnny 5 is alive!");
    } else {
        println!("🤷");
    }

    let number = johnny;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number_wang = if condition {johnny} else {7};
    loop {
        println!("{}! That's number wang!", number_wang);
        break
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter;
        }
    };

    println!("The result, {}, of counter {}.", result, counter);

    let mut count = 0;
    'counting_up: loop {
        loop{
            count += 1;
            break 'counting_up;
        }
    }
    println!("Outer loop label for count {} must start with a single quote?!?", count);

    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}");
        countdown -= 1;
    }
    println!("TO THE MOON!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("while looped array value {} is: {}", index, a[index]);
        index += 1;
    }
    for e in a {
        println!("for looped array value is: {}", e)
    }

    for boom in (1..4).rev() {
        println!("{boom}");
    }
    println!("BOOM?");
}
