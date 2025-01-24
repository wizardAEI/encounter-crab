use std::fs::File;

fn main() {
    let greeting_file_result = File::open("helo.txt");
    if let Ok(f) = greeting_file_result {
        println!("has file {:#?}", f);
    } else if let Err(e) = greeting_file_result{
        println!("err {}", e);
    }
}
