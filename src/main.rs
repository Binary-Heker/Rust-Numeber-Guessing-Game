use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Number Guessing Game!!");
    // Generate Secret Number from 1 to 101
    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("Secret Number {}",secret_num);

    loop {
        println!("Enter the number");
        let mut guess: String = String::new();  
        io::stdin().read_line(&mut guess).expect("Failed to get number");
    
    
        let guess: u32 = guess.trim().parse().expect("Type a number");
        println!("You guessed {}", guess);
    
        match guess.cmp(&secret_num) {
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Less => println!("{}","Too less!".blue()),
            Ordering::Equal => {
                println!("{}","Correct Guess!!".green());
                break;
            }
        }
    }
}
