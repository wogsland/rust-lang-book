fn main() {
    let _v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(5);
    println!("{:?}", v);

    let third: &i32 = &v[2];
    println!("Der Dritte ist {third}.");

    let nineth: Option<&i32> = v.get(8);
    match nineth {
        Some(nineth) => println!("Nine is {nineth}."),
        None => println!("There is no spoon."),
    }

    // bybasssing vector type limits
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Text(String::from("Bueller")),
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Float(3.14),
    ];
    println!("{:?}", row);

    let hello = String::from("こんにちは");
    println!("{hello}");

    let s1 = String::from("malloc");
    let s2 = String::from("realloc");
    let s3 = String::from("free");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blau"), 42);
    scores.insert(String::from("Gelb"), 3);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
