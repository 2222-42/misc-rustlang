use std::fs::File;
fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];

    let f = File::open("hello.txt");
    /*= note: expected type `u32`
    found enum `std::result::Result<File, std::io::Error>`*/
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
}
