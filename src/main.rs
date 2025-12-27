fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Home {:?}", home);
    println!("Home {:?}, {}", home.kind, home.address);
    println!("Loopback {:?}", loopback);
    println!("Loopback {:?}, {}", loopback.kind, loopback.address);

    // Otra forma mas concisa
    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    println!("Home {}", ip_address(&home));
    println!("Loopback {}", ip_address(&loopback));

    // Con otros tipos
    let home = IpAddressType::V4(127, 0, 0, 1);
    let loopback = IpAddressType::V6(String::from("::1"));

    println!("Home: {}", ip_address_type(&home));
    println!("Loopback: {}", ip_address_type(&loopback));

    // Message
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);

    m1.call();
    m2.call();
    m3.call();
    m4.call();
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

fn route(ip_kind: IpAddrKind) {
    println!("Use {:?}", ip_kind);
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

// Otra forma
#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String),
}

fn ip_address(ip_kind: &IpAddress) -> String {
    match ip_kind {
        IpAddress::V4(addr) => addr.to_string(),
        IpAddress::V6(addr) => addr.to_string()
    }
}

// Diff tipo
enum IpAddressType {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn ip_address_type(ip_kind: &IpAddressType) -> String {
    match ip_kind {
        IpAddressType::V4(a1, a2, a3, a4) => format!("{}.{}.{}.{}", a1, a2, a3, a4),
        IpAddressType::V6(addr) => addr.to_string()
    }
}

/*
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
 */

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

/*
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
 */

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quitting the application...");
            }
            Message::Move { x, y } => {
                println!("Moving to position: x={}, y={}", x, y);
            }
            Message::Write(text) => {
                println!("Writing message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to RGB({}, {}, {})", r, g, b);
            }
        }
    }
}
