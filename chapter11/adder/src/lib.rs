pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn test_ne_case() {
        assert_ne!(5, add_two(2))
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail")
    // }
}
