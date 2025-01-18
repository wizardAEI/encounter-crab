fn main() {
    // reference
    let s = String::from("hello");
    let mut len = calculate_length(&s);
    println!("s length: {len}");
    println!("s is valid: {s}");
    let mut s2 = String::from("hello");
    len = calculate_length2(&mut s2); 
    println!("s2 length: {len}");
    s2 = String::from("world");
    len = calculate_length2(&mut s2); 
    println!("s2 length: {len}");
    // we cant borrow a variable as mutable while it is borrowed as mutable or immutable
    // to prevent data races at compile time. 
    let s3 = no_dangle();
    println!("s3: {s3}")
}

fn calculate_length(s: &String) -> usize {
    println!("{:p}", s);
    s.len()
}

fn calculate_length2(s: &mut String) -> usize {
    println!("{:p}", s); // this will be not changed
    s.len()
}

// instead, you are more likely to want to return an owned value
// prevent the dangle pointer
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}