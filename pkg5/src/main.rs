#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;

fn main() {
    let a = [1, 2, 3, 4];
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);    v1.push(2);    v1.push(3);    v1.push(4);
    let v2 = vec![1, 2, 3, 4];
    let second = &v1[1];
    println!("{:?} {}", &v2, second);

    for i in &mut v1 {
        *i += 100 + *i;
    }
    println!("{:?}", &v1);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)        
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue"))
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value)
        }
    }
    
    let mut hmap = HashMap::new();
    hmap.insert(String::from("blue"), 10);
    hmap.insert(String::from("yellow"), 20);
    hmap.insert(String::from("red"), 30);
    hmap.insert(String::from("green"), 40);
    hmap.insert(String::from("orange"), 50);

    println!("\n\n{:?}", hmap);
    for (key, value) in &hmap {
        println!("{}: {}", key, value);
    }
    
    println!("\n");
    let mut hmap = HashMap::new();
    let text = "hello world wonderful world";
    for word in text.split_whitespace() {
        let count = hmap.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}\n", hmap);

    let hmap = HashMap::from([
        ("blue", 10),
        ("yellow", 20),
        ("red", 30),
        ("green", 40),
        ("orange", 50)
    ]);
    println!("{:?}", hmap);
}
