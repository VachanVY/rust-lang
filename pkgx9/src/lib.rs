#![allow(warnings)]
struct Human;

trait Pilot {
    fn fly();
}

trait Wizard {
    fn fly();
}

impl Pilot for Human {
    fn fly() {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly() {
        println!("Up!");
    }
}

impl Human {
    fn fly() {
        println!("*waving arms furiously*");
    }
}

fn main(){
    Human::fly();

    <Human as Pilot>::fly();
    <Human as Wizard>::fly();
}

trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}