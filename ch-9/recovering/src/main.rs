use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    //use_matches();

    //let greeting_file = File::open("hello.txt").unwrap();

    //let greeting_file = File::open("hello.txt").expect("Where's the file?");

    //read_username_from_file();

    //shorter_read_username_from_file();

    //shorterer_read_username_from_file();

    shortest_read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("user.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("user.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn shorterer_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("user.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn shortest_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("user.txt")
}

fn use_matches() {
    let greeting_file_result = File::open("greeting.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("greeting.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Creation of file failed: {:?}", e),
            },
            other_error => {
                panic!("Prob, Bob: {:?}", other_error);
            }
        },
    };
}
