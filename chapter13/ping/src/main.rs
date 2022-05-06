use std::net::Ipv4Addr;

fn ping<A>(addr: A) -> std::io::Result<bool>
where
    A: Into<Ipv4Addr>,
{
    let _ipv4addr = addr.into();
    Ok(true)
}

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type GenericResult<T> = Result<T, GenericError>;

fn parse_i32_bytes(b: &[u8]) -> GenericResult<i32> {
    Ok(std::str::from_utf8(b)?.parse::<i32>()?)
}

fn main() {
    println!("{:?}", ping(Ipv4Addr::new(23, 21, 69, 131)));
    println!("{:?}", ping([66, 146, 231, 89]));
    println!("{:?}", ping(0xd076eb94_u32));

    let result1 = parse_i32_bytes(b"123");
    match result1 {
        Ok(i) => println!("success! {}", i),
        Err(err) => println!("failure! err: {}", err),
    }
    let result2 = parse_i32_bytes(b"test");
    match result2 {
        Ok(i) => println!("success! {}", i),
        Err(err) => println!("failure! err: {}", err),
    }
}
