/* Enums allow to enumerate a List of Variants */
#[derive(Debug)]
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

/* Derive basic Implementations of `Debug` Trait */
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print_hello() {
        println!("Hello");
    }
}

struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

fn main() {
    let _version_four = IpAddressKind::V4;
    let _version_six = IpAddressKind::V6;

    let _localhost = IpAddress {
        kind: IpAddressKind::V4(127,0,0,1),
        address: String::from("127.0.0.1"),
    };

    let x: i8 = 21;
    let y: Option<i8> = Some(42);
    let z: Option<i8> = None;

    let _sum = x + y.unwrap_or(0) + z.unwrap_or(0);

    let a: Option<i32> = Some(42);
    let a_plus_one = plus_one(a);
    println!("a is {}", a_plus_one.unwrap_or(0));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        _ => None
    }
}
