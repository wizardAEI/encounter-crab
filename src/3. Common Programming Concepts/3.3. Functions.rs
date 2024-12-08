fn main() {
    print_labeled_measurement(5, 'h');
    scop_expression();
    let res = plus_one(3);
    println!("3 plus 1 is: {res}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn scop_expression() {
    let y = {
        let x = 1;
        // notice there is no semicolon
        x + 1
    };
    
    println!("the value of y is: {y}");
}

fn plus_one(value: i32) -> i32 {
    value + 1
}