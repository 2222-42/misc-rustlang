use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // tx.send(val).unwrap();
        let result = tx.send(val);
        match result {
            Err(error) => {
                println!("The receiver is dropped: {}", error);
                // to do something to exit thread.
            }
            _ => {
                println!("Sent.");
            }
        };
        // println!("val is {}", val)
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
