trait Speak {
    fn say_hello(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn say_hello(&self) {
        println!("\tWoof!");
    }
}

impl Speak for Cat {
    fn say_hello(&self) {
        println!("\tMeow!");
    }
}

fn greet(speaker: &dyn Speak) {
    speaker.say_hello();
}

// Without dyn (Compile-Time Dispatch)
fn greet_static<T: Speak>(speaker: &T) {
    speaker.say_hello();
}


fn main() {
    let dog = Dog;
    let cat = Cat;

    print!("Dynamic Dispatch\n");
    greet(&dog as &dyn Speak);
    greet(&cat as &dyn Speak);
    print!("Static Dispatch\n");
    greet_static(&dog);
    greet_static(&cat);

    let animals: Vec<Box<dyn Speak>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];
    print!("Dynamic Dispatch with Vec\n");
    for animal in animals {
        animal.say_hello(); // Dynamically dispatched
    }

}
