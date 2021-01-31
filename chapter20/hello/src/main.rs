use std::{io::prelude::*, net::{TcpListener, TcpStream}};

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        
    }
}
