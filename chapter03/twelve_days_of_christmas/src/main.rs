fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let goods = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    let mut count = 0;
    let mut string = String::new();

    for elem in days.iter() {
        println!("On the {} day of Christmas,", elem);
        println!("My true love sent to me");
        string = if count == 1 {
            format!("{}\nand {}", goods[count], string)
        } else {
            format!("{}\n{}", goods[count], string)
        };
        println!("{}", string);
        count = count + 1;
    }
}
