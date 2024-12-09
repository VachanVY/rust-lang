#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = &x; // y is a reference to x // y points to x // y is a memory address of x
    
    let y = Box::new(x); 
    assert_eq!(5, *y);

    let y = MyBox::new(x);
    assert_eq!(5, *y);
    // above and below are same
    assert_eq!(5, *(y.deref())); // y.deref() returns reference // * dereferences the reference

    let m = MyBox::new(String::from("Vachan"));
    hello(&m); // &MyBox<String> -> &String -> &str (string slice)
    hello(&(*m)); 
    hello(&(*m)[..]); // [..] string slice
    yo(&String::from("Vachan"));
    yo("Vachan");
    foo(&String::from("Vachan"));
    // foo("Vachan"); // ERROR: mismatched types &String and &

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    let e = CustomSmartPointer { data: String::from("another stuff") };
    println!("CustomSmartPointers created.");
    // c.drop(); // ERROR: explicit use of destructor method
    // d.drop(); // ERROR: explicit use of destructor method
    println!("e is being dropped manually before the end of main.");
    drop(e); // explicit use of drop function
}
 
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn foo(s: &String) {
    println!("{}", s);
}

fn yo(name: &str) {
    println!("Yo, {}!", name);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
