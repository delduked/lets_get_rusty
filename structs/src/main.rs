use std::ptr::read;

struct User {
    username: String,
    email_addres: String,
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
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("nate@gnated.ca"),
        email_addres: String::from("nate@nated.ca"),
        active: true,
        sign_in_count: 1,
    };

    //    let name: String = user1.username;
    user1.username = String::from("walace");

    let user2: User = build_user(String::from("cam"), String::from("cam@armstrong.com"));

    let user3 = User {
        email_addres: String::from("asdf@fdsa.com"),
        username: String::from("fdsa@asdf.com"),
        ..user2
    };

    struct Point(i32, i32, i32);
    struct Color(i32, i32, i32);

    let width1: u32 = 30;
    let height1: u32 = 50;
    let rect: Rectangle = Rectangle {
        width: 24,
        height: 34,
    };

    println!(
        "the area fo the rectangle is {:#?} square pixles",
        rect.area()
    );

    let rect1: Rectangle = Rectangle {
        width: 23,
        height: 12,
    };

    let rect2: Rectangle = Rectangle {
        width: 50,
        height: 60,
    };

    let rect3 = Rectangle::square(23);

    println!("can rect hold rect 1? {:#?}", rect.can_hold(&rect1));

    println!("can rect hold rec2? {:#?}", rect.can_hold(&rect2));
}

fn build_user(email_addres: String, username: String) -> User {
    User {
        email_addres,
        username,
        active: true,
        sign_in_count: 1,
    }
}
