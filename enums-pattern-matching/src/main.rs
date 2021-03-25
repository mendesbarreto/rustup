
enum IpAddressKind {
    V4(String),
    V6(String),
}

fn print_ip_with_match(value: &IpAddressKind) {
    match value {
        IpAddressKind::V4(value) => println!("This is a IPv4 {} address printed with match", value),
        IpAddressKind::V6(value) => println!("This is a IPv6 {} address printed with match", value),
    }
}

fn print_ip_with_if_let(value: &IpAddressKind) {
    if let IpAddressKind::V4(value) = value {
        println!("This is a IPv4 {} address printed with if let", value)
    } else if let IpAddressKind::V6(value) = value {
        println!("This is a IPv6 {} address printed with if let", value)
    }
}

fn main() {
    let home = IpAddressKind::V4("127.0.0.1".to_string());
    let loopback = IpAddressKind::V6("::1".to_string());

    print_ip_with_match(&home);
    print_ip_with_if_let(&loopback);
}

