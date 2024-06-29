enum IpAddrKing {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Wite(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_funciton() {
        println!("lets get rusty")
    }
}

fn main() {
    let localhost: IpAddrKing = IpAddrKing::V4(127, 0, 0, 1);

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    println!("this is siz {}", six.unwrap());

    let some_value: Option<i32> = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_value {
        println!("three")
    }
}

fn route(ip_kind: IpAddrKing) {}

struct IpAddr {
    kind: IpAddrKing,
    address: String,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}
