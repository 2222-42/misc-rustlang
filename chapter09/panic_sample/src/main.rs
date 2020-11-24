use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_use_question() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    println!("not failed on reading.");
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];

    let f = File::open("hello.txt");
    /*= note: expected type `u32`
    found enum `std::result::Result<File, std::io::Error>`*/
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => {
    //             panic!("Tried to create file but there was a problem: {:?}", e)
    //         }
    //     },
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     }
    // };
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    let result = read_username_from_file();
    match result {
        Ok(v) => println!("use name is {}",v),
        Err(e) => {println!("There was a problem opening the file: {:?}", e);}
    }

    let result = read_username_from_file_use_question();
    match result {
        Ok(v) => println!("use name is {}",v),
        Err(e) => {println!("There was a problem opening the file: {:?}", e)}
    }
}
