fn foo(_: i32, y: i32) {
    println!("This code only use the y parameter: {}", y);
}

fn main() {
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    match numbers {
        // (.., second, ..) => {
        //     println!("Some number: {}", second);
        // }
        (_, second, ..) => {
            println!("Some number: {}", second);
        }
    }

    let x = 5;
    let _y = 10;

    let s = Some(String::from("hello"));
    // if let Some(_s) = s {
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s)
}
