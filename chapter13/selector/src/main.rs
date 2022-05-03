use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

fn main() {
    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };

    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());
    *s = 'w';
    assert_eq!(s.elements, vec!['x', 'y', 'w']);

    let t = Selector {
        elements: vec!["good", "bad", "ugly"],
        current: 2,
    };
    fn show_it(thing: &str) {
        println!("{}", thing);
    }
    show_it(&t);

    fn show_generics<T: Display>(thing: T) {
        println!("{}", thing);
    }
    // show_generics(&t); // deref coercionsがなされない
    show_generics(&*t);
    show_generics(&t as &str);
}
