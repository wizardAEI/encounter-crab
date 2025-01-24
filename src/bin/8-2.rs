// Storing UTF-8 Encoded Text with String
fn main() {
    // create and edit
    let mut s1 = String::from("foo");
    let s2 = "bar";
    println!("s2 is {s2}");
    s1.push_str(s2);
    s1.push('x');
    println!("s1 push s2 and x: {}", s1);
    let s3 = format!("x = {},\n y = {val}", 10, val = 30);
    println!("{}", s3);
    
    // + operator
    let s1 = String::from("Hello, ");
    let mut s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    s2.push('x');
    println!("s2: {}", s2);
    println!("s3: {}", s3);

    let s1 = "123";
    let s2 = "456";
    let s3 = format!("{s1} + {s2}");
    println!("{s3}");

    let s1 = "哦啤酒啊";
    let first = s1.chars().nth(1);
    if let Some(ch) = first {
        println!("the first char of s1 is {:?}", ch)
    }

}