// ENUMS
// Standard library has this exact thing. Below is just for illustration purposes
// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let four = IpAddrKind::V4(127, 0, 0, 1);
//     let six = IpAddrKind::V6(String::from("::1"));
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let cents = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("cents: {cents}");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(six);
}

fn value_in_cents(coin: Coin) -> u8 {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state}");
    } else {
        count += 1;
    }

    dbg!(count);

    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
