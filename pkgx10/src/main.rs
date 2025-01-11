#![allow(warnings)]
use std::fmt;

struct Wrapper(Vec<String>);

// function pointers
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    return f(arg) + f(arg)
}

fn do_twice_Fn<T>(f: T, arg:i32) -> i32
    where T: Fn(i32) -> i32 {
    return f(arg) + f(arg)
}

// No two closures, even if identical, have the same type... 
// so we can't return a closure directly... we can use a trait object... 
// inside a Box
fn return_closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        return Box::new(move |x| x + a);
    } else {
        return Box::new(move |x| x - a);
    }
}

fn main(){
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    use pkgx10::vector;
    let v = vector![1, 2, 3];
    println!("{:?}", v);
}

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro; 

#[derive(HelloMacro)]
struct Pancakes;

