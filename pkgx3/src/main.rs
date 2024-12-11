// A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T> lets you have 
// multiple owners of some data, but it only gives immutable access to that data. 
// If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners 
// and that you can mutate!

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<String>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(String::from("value")));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(String::from("b"))), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(String::from("c"))), Rc::clone(&a));

    println!("a before = {a:?}");
    println!("b before = {b:?}");
    println!("c before = {c:?}\n\n");

    *value.borrow_mut() = String::from("MUTATED VALUE");

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}\n\n");

    let a = Rc::new(RefCell::new(1));
    let b = Rc::clone(&a);
    println!("|| a = {} || b = {} ||", a.borrow(), b.borrow());
    *b.borrow_mut() += 1;
    println!("|| a = {} || b = {} ||", a.borrow(), b.borrow());

    /*
    int* a = 1;
    int* b = a;
    printf("|| a = %d || b = %d ||\n", *a, *b);
    */
}