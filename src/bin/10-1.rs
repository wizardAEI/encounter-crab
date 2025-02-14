struct Point<U, T> {
    x: U,
    y: T
}

impl<U, T> Point<U, T> {
    fn x(&self) -> &U {
        &self.x
    }
}

// imply f32
impl Point<i32, f64> {
    fn sum(&self) -> f64 {
        let x = self.x as f64;
        x + self.y
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];
    for item in list {
        if largest < item {
            largest = item;
        }
    }
    largest
}

fn largest_i32(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let result = largest(&char_list);
    println!("the largest char with largest is {result}");

    let location = Point{
        x: 1,
        y: 2.0,
    };

    println!("the x of location is {}", location.x());
    println!("the sum of location is {}", location.sum());
}