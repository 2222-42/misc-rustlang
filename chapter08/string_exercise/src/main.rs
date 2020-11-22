fn main() {
    let text1 = "first";
    let result = pig_latin(text1);
    println!("{}", result);

    let text2 = "apple";
    let result = pig_latin(text2);
    println!("{}", result);
}

fn pig_latin(str: &str) -> String {
    let s = &str[0..1];
    let rest = &str[1..];
    match s {
        "a" => format!("{}-hay", str),
        "i" => format!("{}-hay", str),
        "u" => format!("{}-hay", str),
        "e" => format!("{}-hay", str),
        "o" => format!("{}-hay", str),
        _ => format!("{}-{}ay", rest, s),
    }
}
