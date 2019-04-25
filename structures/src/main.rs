#![allow(unused_variables)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
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
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

