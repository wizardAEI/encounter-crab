#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let r = Rectangle {
        width: dbg!(20 * 2), // Prints and returns the value of a given expression for quick and dirty debugging.

        height: 30
    };
    let area = calculate_area(&r);
    println!("area is: {}", area);
    dbg!(r);
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}