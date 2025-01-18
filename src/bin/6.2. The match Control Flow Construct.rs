enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let v = Coin::Penny;

    let value = value_in_cents(v);

    println!("value is: {}", value);

    // matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // match some special values and a other arm
    let dice_roll = 9;
    let mut fancy_hat = 1;
    match dice_roll {
        3 => fancy_hat = fancy_hat + 1,
        7 => fancy_hat = fancy_hat - 1,
        // oor other => ()
        _ => (),
    };
    println!("fancy hat number: {}", fancy_hat)
}

fn value_in_cents(coin: Coin) -> u8 {
    // match must be covered all possibilities
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // return match result
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}