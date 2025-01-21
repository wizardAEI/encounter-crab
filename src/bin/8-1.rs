// Storing Lists of Values with Vectors

fn main() {
    let mut  v: Vec<isize> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    println!("vector: {:?}", v); // vector: [1, 2]

    v[2] = 10;
    println!("vector: {:?}", v);

    let third = &mut v[2];

    *third += 10;

    println!("vector: {:?}", v);

    let third = v.get(2);

    match third {
        Some(v) => {
            println!("The third element is {}", v);
        },
        None => {
            println!("There is no third element.")
        },
    }

    let tenth = v.get(10);

    match tenth {
        Some(v) => {
            println!("The tenth element is {}", v);
        },
        None => {
            println!("There is no tenth element.")
        },
    };

    for i in &mut v {
        *i += 50;
    }

    println!("vector: {:?}", v);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String, String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue"), String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[0] {
        SpreadsheetCell::Int(v) => {
            println!("Int {}", v);
        },
        SpreadsheetCell::Float(v) => {
            println!("Float {}", v);
        },
        SpreadsheetCell::Text(str1, str2) => {
            println!("Text {}, {}", str1, str2);
        },
    }
}    