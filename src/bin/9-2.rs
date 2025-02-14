use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // example();
    // example2();
    // example3();
    let file_res = read_username_from_file();
    match file_res {
        Ok(_) => println!("ok"),
        Err(_) => {
            let f = File::create("hello.txt");
            println!("err and create a new file {:#?}", f);
        },
    }
}

fn example() {
    let greeting_file_result = File::open("hello.txt");
    if let Ok(f) = &greeting_file_result {
        println!("has file {:#?}", f);
    } else if let Err(e) = &greeting_file_result{
        println!("err {}", e);
    }
    // or
    let greet_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            println!("err: {:#?}", error);
            // create file
            File::create("hello.txt").expect("Failed to create file")
        },
    };
    println!("greet_file: {:#?}", greet_file);
}

fn example2() {
    let greeting_file_result = File::open("hello2.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello2.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    println!("file: {:#?}", greeting_file);
}

fn example3() {
    File::open("hello.ext").expect("no this file");
}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    // the ? operator is defined to perform an early return of a value out of the function
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    // the ? operator is defined to perform an early return of a value out of the function
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}