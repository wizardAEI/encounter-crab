fn main() {
    // ownership
    let s1 = String::from("hello");
    let s2 = s1; // s1's value move
    println!("s2: {s2}");
    let s3 = take_ownership(s2); // s2's value move into the function and so is no longer valid here
    // println!("s1: {s1}, s2: {s2}"); // borrow of moved value: `s1`
    println!("s3: {s3}")
}

fn take_ownership(s: String) -> String {
    println!("s: {}", s);
    s
}