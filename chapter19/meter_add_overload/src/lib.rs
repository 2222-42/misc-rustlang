use std::ops::Add;

#[derive(Debug, PartialEq)]
struct MilliMeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for MilliMeters {
    type Output = MilliMeters;

    fn add(self, other: Meters) -> MilliMeters {
        MilliMeters(self.0 + (other.0 * 1000))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_works() {
        let milli = MilliMeters(1000);
        let m = Meters(1);
        assert_eq!(milli + m, MilliMeters(2000));
    }
}
