enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn Call(&self) {
        
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));

    m.Call();

    // Option<T>
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number:Option<i32> = None;

    let mut a = 'x';
    if let Some(c) = some_char {
        a = c;
    };
    // the Option<T> help us to eliminate the risk of incorrectly assuming a not-null value
}