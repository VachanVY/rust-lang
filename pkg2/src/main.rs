fn main() {
    // - Ownership rules -
    // Each value in Rust has a variable that's called its owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.*/
    // The Rules of References
    // 1. At any given time, you can have either one mutable reference
    // or any number of immutable references.
    // 2. References must always be valid.

    { // s is not valid here
        let _s = String::from("hello"); // s is valid from this point
    } // scope of s is over 

    // ON STACK
    let x = 5;
    let _y = x; // copy

    // ON HEAP
    let s1 = String::from("hello"); 
    println!("(0) {}", s1);
    // let s2 = s1;
    // println!("(1) {}", s1); // ERROR!!! Ownership transfered to s2
    let s2 = s1.clone();
    println!("(0) {} , {}", s1, s2);

    // makes_copy
    makes_copy(x);
    println!("x: {}", x); // can use it again

    // takes_ownership when dynamically allocated
    takes_ownership(s1);
    // println!("(0) {}", s1); // ERROR!!! ownership transfered to temp var inside the function

    let mut strng = String::from("Yo tell me what you want... what you really really want");
    let len = strlen(&strng);
    println!("len: {}", len);

    string_make_title(&mut strng);
    string_make_title(&mut strng); // can have mutliple refernces in different scopes

    println!("strng: {}", strng);

    let mut s = String::from("yo");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2); // ERROR!!! cannot have multiple mutable references

    // can have multiple immutable references 
    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2); 
    // can't use r1 and r2 after this point, so can make a mutable referenceafter this

    let r3 = &mut s;
    println!("{}", r3);
}

fn takes_ownership(some_string: String){
    println!("some_string takes ownership {}", some_string);
}

fn makes_copy(some_int: u32){
    println!("makes_copy => some_int: {}", some_int);
} 

fn _takes_and_gives_back(a_string: String) -> String {
    return a_string; // meh, we dont want to do this
}

fn strlen(s: &String) -> usize {
    let len = s.len();
    return len;
}

fn string_make_title(s: &mut String){
    s.insert_str(0, "|| ");
    s.push_str(" ||");
}