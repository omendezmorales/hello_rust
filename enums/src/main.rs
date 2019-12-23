#![allow(unused_variables)]

fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => {
                 println!("Lucky Nickel!");
                 5
            },
            Coin::Dime => {
                println!("Lucky Dime!");
                10},
            Coin::Quarter => {
                println!("Lucky Quarter!");
                25},
        }
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some number:{:?} ", some_number);
    println!("some string: {:?}", some_string);
    println!("printing in ip address in v6 format {}", loopback.address);
    println!("printing in ip address in v4 format {}", home.address);
    
    let a_coin = value_in_cents(Coin::Quarter);
}
