#![allow(unused_variables)]

fn main() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    enum Coin {
        #[warn(dead_code)]
        _Penny,
        _Nickel,
        _Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::_Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::_Nickel => {
                println!("Lucky Nickel!");
                5
            }
            Coin::_Dime => {
                println!("Lucky Dime!");
                10
            }
            Coin::Quarter => {
                println!("Lucky Quarter!");
                25
            }
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

    #[warn(dead_code)]
    enum _Message {
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
    println!(
        "printing in ip address in {:?} format: {}",
        home.kind, home.address
    );
    println!(
        "printing in ip address in {:?} format: {} ",
        loopback.kind, loopback.address
    );

    let a_coin = value_in_cents(Coin::Quarter);
    let some_u8_value = 2u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    println!("heres what some_u8_value is: {}", some_u8_value);
}
