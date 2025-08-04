enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
    nickname: Option<String>, // Added Option type here
}

fn route(ip_kind: IpAddrKind) {
    // function body can be empty for now
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("127.0.0.1")),
        address: String::from("127.0.0.1"),
        nickname: Some(String::from("Localhost")), // Using Some variant
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),
        address: String::from("::1"),
        nickname: None, // No nickname provided
    };

    // Example usage of Option
    match &home.nickname {
        Some(name) => println!("Home nickname: {}", name),
        None => println!("Home has no nickname"),
    }

    match &loopback.nickname {
        Some(name) => println!("Loopback nickname: {}", name),
        None => println!("Loopback has no nickname"),
    }

    // Example usage of the function
    route(home.kind);
    route(loopback.kind);
}
