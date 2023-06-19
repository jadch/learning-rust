enum IpAddressKind {
    V4,
    V6,
}

struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

fn main() {
    let four: IpAddressKind = IpAddressKind::V4;
    let six: IpAddressKind = IpAddressKind::V6;

    let localhost: IpAddress = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };
}
