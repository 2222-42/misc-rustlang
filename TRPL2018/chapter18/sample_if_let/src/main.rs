fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // let Some(color) = favorite_color;

    // if let x = 5 {
    //     println!("{}", x);
    // }

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background");
        } else {
            println!("Using orange as the background");
        }
    } else {
        println!("Using blue as the background");
    }
}
