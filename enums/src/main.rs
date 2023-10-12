// we could also use a struct as a type here
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// enums arent by default prescribed data so we might try to use a struct to assign some, but we
// can do something as shown above and assign a type to it
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
fn main() {
    // simple enums
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let m = Message::Write(String::from("hello"));
    m.call();

    // struct implementation
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6("::1".to_string());

    //options
    //enum Option<T> {
    //  None,
    //  Some(T)
    //}

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // error because you cant assume that there exists a value
    // let sum = x + y;
}

// can use either variant
fn route(ip_kind: IpAddrKind) {}

