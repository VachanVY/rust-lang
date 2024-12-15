#![allow(warnings)]

/*
> Unsafe Rust!!!
    * Dereference a raw pointer
    * Call an unsafe function or method
    * Access or modify a mutable static variable
    * Implement an unsafe trait
    * Access fields of unions
*/

#[link(name = "example")] // Links the C library (omit "lib" prefix)
extern "C" {
    fn say_hello();           // Declare the C function
    fn add(a: i32, b: i32) -> i32; // Declare the C function with parameters and return type
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    return unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32; // *const: immutable raw pointer
    let r2 = &mut num as *mut i32; // *mut: mutable raw pointer

    unsafe {
        *r2 = 10;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        // Call the C functions within an unsafe block
        say_hello();
        let result = add(5, 7);
        println!("The result of add(5, 7) is: {}", result);
    }

    println!("{}", HELLO_WORLD);
}
























/*  GITHUB COPILOT: Can't confirm 
    * Call a function over FFI
    * Access a mutable or static variable without synchronization
    * Implement a foreign function interface (FFI) function
    * Access a field of a struct that uses the repr(transparent) attribute
    * Access a field of a struct that uses the repr(packed(N)) attribute
    * Call a function declared within an extern block
    * Use the asm! macro
    * Use the union field offset calculation macro
    * Use the union field offset calculation function
    * Use the union field alignment calculation macro
    * Use the union field alignment calculation function
    * Use the union size calculation macro
    * Use the union size calculation function
    * Use the transmute function
    * Use the from_raw and from_raw_parts methods
    * Use the slice::from_raw_parts function
    * Use the slice::from_raw_parts_mut function

*/