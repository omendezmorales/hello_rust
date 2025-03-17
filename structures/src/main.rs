
#![allow(unused_variables)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    }


#[derive(Debug)] 
 struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }


    //from the documentation>
    // we’re allowed to define functions within impl blocks that DON’T TAKE self as a parameter
    fn square(size: u32) -> Rectangle { 
        Rectangle{width:size, height:size}
    }
}


fn main() {


let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
    };

let mut user2 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 2,
};

user2.email = String::from("anotheremail@example.com");    

let user3 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count + 1 ,
};

let user4 = build_user(String::from("mmoa33@hotmail.com"), String::from("orco"));

println!("User's 1 email: {}", user1.email);
println!("User's 3 sign_in: {}", user3.sign_in_count);
println!("User's 4 email: {}, user-id: {}", user4.email, user4.username);

let rect1 = Rectangle { width: 30, height: 50 };
let rect2 = Rectangle { width: 10, height: 40 };
let rect3 = Rectangle { width: 60, height: 45 };
let sq = Rectangle::square(30); //notice the :: invoking syntax instead of using .

let rect2_t = (30,50);
    println!(
        "The area of the rectangle is {} square pixels, using method syntax",
        // area(&rect1)
        rect1.area()
    );

    println!("rect1 is {:?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Area with a rectangle using the square method: {}", sq.area());
    
    println!(
        "The area of the rectangle is {} using a tuple.",
        area_t(rect2_t)
    );
    println!(
        "The area of the rectangle 1  is {}.",
        area(&rect1)
    );

}//end of main

fn area_t(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}



fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

