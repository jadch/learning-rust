enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

// enum Option<T> {
//     Some(T),
//     None,
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function() {
        println!("Hey this is the Message.some_function()")
    }
}

fn main() {
    let localhost: IpAddressKind = IpAddressKind::V4(127, 0, 0, 1);
}
