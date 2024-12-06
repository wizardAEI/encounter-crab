fn main() {
    // scalar types: integers, floating, number, booleans, characters
    
    // integer: i/u 8 16 32 64 128 arch(isize usize)
    let a: i8 = 21;
    let b: isize = 256;
    // check overflow
    let c = a.checked_add(125);
    match c {
        Some(res) => println!("res {res}"),
        None => println!("21 + 125 overflow when a is i8")
    }
    println!("i8 example: {a} and isize example: {b}");

    // floating-point types i64 by default
    let x = 2.0;
    println!("float value x: {x}");

    // boolean
    let b = false;
    println!("boolean b: {b}");

    // char
    let emoji = '🚀';
    println!("char emoji: {emoji}");

    // compound types
    // tuple
    let tip = (50, 6.4, 1);
    let (x, y, z) = tip;
    println!("coordinate: {x},{y},{z}");
    let x = tip.1;
    println!("tip.1: {x}");
    // unit
    let x = {};
    println!("unit value: {:?}", x);
    // array
    let arr = [1, 2];
    println!("arr: {:?}", arr);
}
