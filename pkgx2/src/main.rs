#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main(){
    // https://doc.rust-lang.org/stable/alloc/rc/index.html

    // The type Rc<T> provides shared ownership of a value of type T, allocated in the heap.
    // Invoking clone on Rc produces a new pointer to the same allocation in the heap. 
    // When the last Rc pointer to a given allocation is destroyed, the value stored in that 
    // allocation (often referred to as “inner value”) is also dropped.

    // Shared references in Rust disallow mutation by default, and Rc is no exception: you 
    // cannot generally obtain a mutable reference to something inside an Rc. If you need 
    // mutability, put a Cell or RefCell inside the Rc; see an example of mutability inside an Rc.

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    
    // let a = 5;
    // let b = &mut a; ERROR: cannot borrow `a` as mutable, as it is not declared as mutable

    // let mut c = 10;
    // let d = &c;
    // *d = 20; ERROR: cannot assign to `*d` because it's not a mutable reference
}


/* CAN'T DO THIS BECAUSE WE ARE TRANSFERRING OWNERSHIP OF a TO b (AND THEN TO c WHICH IS NOT ALLOWED)
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a)); 
}
*/
