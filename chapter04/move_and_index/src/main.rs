fn main() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // let third = v[2];
    // let fifth = v[4];

    let fifth = v.pop().expect("vector empty");
    assert_eq!(fifth, "105");

    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    let _first = &v[1];

    assert_eq!(v, vec!["101", "104", "substitute"]);

    struct Person {
        name: Option<String>,
        _birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Johannes Brahms".to_string()),
        _birth: 1833,
    });

    // let first_name = composers[0].name
    let first_name = composers[0].name.clone();
    assert_eq!(first_name, Some("Johannes Brahms".to_string()));

    let first_name = std::mem::replace(&mut composers[0].name, None);
    assert_eq!(first_name, Some("Johannes Brahms".to_string()));
    assert_eq!(composers[0].name, None);
}
