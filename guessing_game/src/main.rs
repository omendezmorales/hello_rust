extern crate rand;
use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    loop {
    println!("guess the number.");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
    let mut guess= String::new();
           
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");
    
    let guess: u32= match guess.trim().parse()
     {
        Ok(num) => num,
        Err(_) => continue,
    };
                    // .expect("Please type a number!");
    println!("you guessed: {}", guess);    
     println!("The secret number is: {}", secret_number);

    
        //println!("Please input your guess.");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
