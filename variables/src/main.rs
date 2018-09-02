// fn main() {
    
    // const MAX_POINTS: u32 = 100_000;

    // let mut x=5;
    // println!("the value of x is {}", x);
    // x=6;
    // println!("the value of x is {}", x);

//}

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let x = 5;
    let y: f32 = 3.0;
    let x = x + 1;
    let x = x * 2;
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let index = 100;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of tuple is: {},{},{}", tup.0, tup.1,tup.2);
    println!("The 1st value of months is: {}", months[index]); //interesting error message when running it.
}

