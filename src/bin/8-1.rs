// Storing Lists of Values with Vectors

use std::collections::HashMap;

fn main() {
    let mut v: Vec<isize> = Vec::new();

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
        }
        None => {
            println!("There is no third element.")
        }
    }

    let tenth = v.get(10);

    match tenth {
        Some(v) => {
            println!("The tenth element is {}", v);
        }
        None => {
            println!("There is no tenth element.")
        }
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

    // match 会尝试移动 row[0] 的所有权，但是后续在 match 里还会使用 row[0]，所以不使用移动所有权而是引用
    match &row[0] {
        SpreadsheetCell::Int(v) => {
            println!("Int {}", v);
        }
        SpreadsheetCell::Float(v) => {
            println!("Float {}", v);
        }
        SpreadsheetCell::Text(str1, str2) => {
            println!("Text {}, {}", str1, str2);
        }
    }

    let mut arr = vec![5, 6, 7, 1, 2, 2, 4];
    arr.sort();
    let middle_index = arr.len() / 2;
    println!("7 / 2: {}", middle_index);
    let median = arr[middle_index];
    println!("median {}", median);
    let mut map = HashMap::new();
    for key in arr {
        map.entry(key).and_modify(|v| *v += 1).or_insert(1);        
    }
    #[derive(Debug)]
    struct Mode {
        v: i32,
        times: u32
    }
    let mut mode = Mode {
        v: 0,
        times: 0,
    };
    for (key, v) in map.iter() {
        if mode.times < *v {
            mode.times = *v;
            mode.v = *key;
        }
    };
    println!("mode: {:#?}", mode);
}
