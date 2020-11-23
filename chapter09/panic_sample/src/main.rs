use std::{fs::File, io::ErrorKind};
fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];

    let f = File::open("hello.txt");
    /*= note: expected type `u32`
    found enum `std::result::Result<File, std::io::Error>`*/
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Tried to create file but there was a problem: {:?}", e)
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
