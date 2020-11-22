fn main() {
    let text1 = "first";
    let result = pig_latin(text1);
    println!("{}", result);
    let result = pig_latin_use_char(text1);
    println!("{}", result);

    let text2 = "apple";
    let result = pig_latin(text2);
    println!("{}", result);
    let result = pig_latin_use_char(text2);
    println!("{}", result);
}

fn pig_latin(str: &str) -> String {
    let s = &str[0..1];
    let rest = &str[1..];
    match s {
        "a" | "i" | "u" | "e" | "o" => format!("{}-hay", str),
        _ => format!("{}-{}ay", rest, s),
    }
}

fn pig_latin_use_char(str: &str) -> String {
    let mut chars = str.chars();
    let s = chars.nth(0);
    match s {
        Some('a') | Some('i') | Some('u') | Some('e') | Some('o') => format!("{}-hay", str),
        Some(v) => {
            let mut rest = String::new();
            for elem in chars {
                rest = format!("{}{}", rest, elem)
            }
            format!("{}-{}ay", rest, v.to_string())
        }
        None => format!(""),
    }
}
