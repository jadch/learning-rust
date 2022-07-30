enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

fn main() {
    let localhost: IpAddressKind = IpAddressKind::V4(127, 0, 0, 1);
}
