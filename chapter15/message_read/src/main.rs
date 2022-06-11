fn main() {
    let message = "To: jimb\r\n\
    From: id\r\n\
    \r\n\
Ooooh, donuts!!\r\n";

    let mut lines = message.lines();
    println!("Headers: ");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }

    println!("\nBody: ");
    for line in lines {
        println!("{}", line);
    }
}
