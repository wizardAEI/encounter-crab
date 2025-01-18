fn main() {
    // if
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // multiple conditions
    if number % 4 == 0 {
        println!("number is divisible by 4")
    } else if number % 3 == 0 {
        println!("number is divisible by 3")
    } else {
        println!("number is not divisible by 4 or  3")
    }

    let number = if number > 3 { 4 } else { 2 };
    println!("number : {}", number);

    // repetition with loop
    let mut number = 4;
    let result = loop {
        println!("again");
        if number < 0 {
            break number;
        } else {
            number -= 1;
        }
    };
    println!("loop result: {result}");

    // loop labels
    let mut count = 0;
    // single quote + label name
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1
        }

        count += 1;
    }
    println!("End count = {count}");

    // while
    let mut number = 3;
    while number != 0 {
        println!("number {number}");
        number -= 1;
    }

    // for in
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is : {element}")
    }
    for number in 1..4 {
        println!("number: {number}");
    }
    for number in (1..4).rev() {
        println!("number: {number}");
    }
}
