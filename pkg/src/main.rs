use std::{cmp::Ordering, io};
use colored::Colorize;
use rand::Rng;

fn main(){
    // what's wrong?
    let secret_number = rand::thread_rng().gen_range(0..10); 

    loop{
        println!("Guess the number!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        println!("Secret Number: {}", secret_number);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Small!".red()), 
            Ordering::Equal => {
                println!("{}", "Equal! You win!!!".green());
                break;
            },
            Ordering::Greater => println!("{}", "Big!".red())
        }
    }
}
