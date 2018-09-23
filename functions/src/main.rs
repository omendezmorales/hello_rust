extern crate rand;
use rand::Rng;

fn main() {
    println!("Hello, main!");
    let x = rand::thread_rng().gen_range(1, 101);
    //another_function(x, x*x);
    let f = five();
    println!("callings five-> {}", f);
    //let x = (let y = 6);
}

fn five() -> i32 {
    5
}

fn another_function(x:i32, y:i32) {
    println!("got this as input arguments {},{}.", x, y);
}
