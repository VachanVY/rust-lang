#![allow(warnings)]

use core::num;

#[derive(Debug)]
enum Languages {
    Rust,
    Python,
    C,
    Cpp
}

fn main() {
    let lang = Languages::Rust;
    match lang {
        Languages::Rust => println!("Rust"),
        Languages::Python => println!("Python"),
        Languages::C => println!("C"),
        lang => println!("{:?}", lang)
    }

    // Conditional if let
    let authorization_status: Option<&str> = Some("Authorized");
    if let Some("Authorized") = authorization_status {
        println!("Authorized");
    } else {
        println!("Not authorized");
    }

    // Conditional while let
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    print_coordinates(&(3, 5));

    let point = Point { x: 3, y: 5 };
    let Point { x: a, y:b } = point;
    print_coordinates(&(a, b));

    match point {
        Point {x, y: 0} => println!("On the x-axis at {}", x),
        Point {x: 0, y} => println!("On the y-axis at {}", y),
        Point {x, y} => println!("On neither axis at ({}, {})", x, y)
    }

    let numbers = (2, 4, 6, 8, 10);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => println!("None")
    }

    // @ operator
    println!("\n\nUsing @ operator");
    let x = 5;
    match x {
        e @ 1..=5 => println!("Got a range element: {}", e),
        _ => println!("Got something else")
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

struct Point {
    x: i32,
    y: i32
}