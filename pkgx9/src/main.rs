#![allow(warnings)]
use std::ops::Add;

struct Human;

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, 
        Point { x: 3, y: 3 }
    );

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct Counter {}

/*
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;    
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        return Some(1);
    }
}
*/

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;    
}

impl Iterator<i32> for Counter {
    fn next(&mut self) -> Option<i32> {
        return Some(1);
    }    
}
impl Iterator<f64> for Counter {
    fn next(&mut self) -> Option<f64> {
        return Some(1.0);
    }    
}