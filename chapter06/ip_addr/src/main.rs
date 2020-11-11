enum IpAddrKind {
    v4(u8, u8, u8, u8),
    v6(String),
}

fn main() {
    let home = IpAddrKind::v4(127, 0, 0, 1);
    let loopback = IpAddrKind::v6(String::from("::1"));
    println!("Hello, world!");
    route(home);
    route(loopback);
}

fn route(ip_type: IpAddrKind) {}
