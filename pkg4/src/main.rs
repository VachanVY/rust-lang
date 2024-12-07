#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

enum _IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String)
}

enum _Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl _Message {
    fn _some_number(){
        println!("Hello");
    }    
}

// let _localhost = IpAddrKind::V4(127, 0, 0, 1);

// // some_message
// let _some_message = Message::Write(String::from("Hello"));
// let _some_message = Message::Move{x: 1, y: 2};
// let _some_message = Message::ChangeColor(1, 2, 3);
// println!("Done");

fn main() {
    let _some_num: Option<i32> = Some(5);
    let _some_string: Option<String> = Some(String::from("Hello"));
    let _absent_num: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(7);
    let z: Option<i32> = None;

    // let sum = x + y; // Error because y is Option<i32> and x is i32 so we can't add them directly
    let sum = x + y.unwrap_or(0) + z.unwrap_or(10);
    println!("Sum: {}", sum);
 
    let _coin = value_in_cents(Coin::Quarter(UsState::California));

    plus_one(Some(5));
    plus_one(None);    
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    return match x {
        Some(i) => Some(i + 1),
        // wrap it in some because we can't return i32 directly
        // we are returning Option<i32> so we have to wrap it in Some

        _ => None // _ means catch all other cases
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Ohio,
    California
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
