fn main() {
    let _quatro = IpAddrKind::V4;
    route(IpAddrKind::V6);
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loop = IpAddr::V6(String::from("::1"));

    let _some_number = Some(1337);
    let _absent_number: Option<i32> = None;

    value_in_cents(Coin::Penny);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("This syntax with {} is confusing.", max);
    }
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(_ip_kind: IpAddrKind) {}

enum IpAddr {
    V4(String),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Find a penny. Pick it up...");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
