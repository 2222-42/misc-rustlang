use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
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
            thread::sleep(Duration::from_secs(1));
            // println!("val is {}", val)}
        }
    });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    for received in rx {
        println!("Got: {}", received);
    }
}
