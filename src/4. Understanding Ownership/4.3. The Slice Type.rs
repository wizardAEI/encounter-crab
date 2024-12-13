fn main() {
    // slice
    let mut s = String::from("hello");
    let word = first_word(&s); // there s have borrowed as immutable
    // s.clear(); // cannot borrow `s` as mutable because it is also borrowed as immutable (via slice)
    println!("word: {word}");
    s.clear(); // that's ok, because borrowed end
    println!("s: {s}")
 }
 
 fn first_word(s: &String) -> &str {
     let chars = s.chars();
 
     for (i, c) in chars.enumerate() {
         if c == ' ' {
             return  &s[..i];
         }
     }
 
     return  &s[..];
 }