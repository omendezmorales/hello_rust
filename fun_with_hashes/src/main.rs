extern crate fun_with_hashes;
use fun_with_hashes::solution;
use std::io;

fn main() {
    println!("Please input your hash length.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,

        Err(_) => 0,
    };
    let hash_found: u32 = solution(guess);
    println!("this is a hash: {:?}", hash_found);
}
