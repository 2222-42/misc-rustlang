use std::collections::HashMap;

fn main() {
    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring",7_762);
    populations.insert("The Dalles", 15_340);

    assert_eq!(populations.iter().max_by_key(|&(_name, pop)| pop), Some((&"Portland", &583_776)));
    assert_eq!(populations.iter().min_by_key(|&(_name, pop)| pop), Some((&"Greenhorn", &2)));
    assert_eq!(populations.iter().find(|&(_name, pop)| pop > &1_000_000), None);
    assert_eq!(populations.iter().find(|&(_name, pop)| pop > &500_000), Some((&"Portland", &583_776)));
}
