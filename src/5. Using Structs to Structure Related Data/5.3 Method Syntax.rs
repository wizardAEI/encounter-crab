
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn change_width(&mut self, width: u32) {
        self.width = width;
    }
    fn test(arg1: usize) -> u128{
        match u128::try_from(arg1) {
            Ok(val) => {
                return val + 123;
            },
            Err(_) => {
                return 0;
            },
        };
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut r = Rectangle {
        width: dbg!(20 * 2), // dbg! will returns ownership of the expression's value
        height: 30
    };
    let area = calculate_area(&r);
    println!("area is: {}", area);
    r.change_width(50);
    let area = r.area();
    println!("area2 is: {}", area);
    dbg!(&r);
    let r2 = Rectangle::square(12);
    dbg!(&r2);
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}