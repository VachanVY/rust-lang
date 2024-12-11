#![allow(warnings)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn yoo() {
    let shared_value = Rc::new(RefCell::new(String::from("Hap")));

    let owner1 = Rc::clone(&shared_value);
    let owner2 = Rc::clone(&shared_value);

    ((*owner1).borrow_mut()).push_str("py");
    ((*owner2).borrow_mut()).push_str(" Birthday ");
    ((*shared_value).borrow_mut()).push_str("Vachan");

    println!("shared_value = {:?}", shared_value.borrow());
    println!("owner1 = {:?}", owner1.borrow());
    println!("owner2 = {:?}", owner2.borrow());
}


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
