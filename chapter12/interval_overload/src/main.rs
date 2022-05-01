use std::cmp::{Ordering, Reverse};

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T,
    upper: T,
}

impl<T: PartialOrd> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        } else if self.lower >= other.upper {
            return Some(Ordering::Greater);
        } else if self.upper <= other.lower {
            return Some(Ordering::Less);
        } else {
            return None;
        }
    }
}

fn main() {
    assert!(
        Interval {
            lower: 10,
            upper: 20
        } < Interval {
            lower: 20,
            upper: 40
        }
    );
    assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1 });
    assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8 });

    let left = Interval {
        lower: 10,
        upper: 30,
    };
    let right = Interval {
        lower: 20,
        upper: 40,
    };
    assert!(!(left < right));
    assert!(!(left >= right));

    let mut intervals = vec![
        Interval {
            lower: 10,
            upper: 20,
        },
        Interval { lower: 7, upper: 8 },
        Interval {
            lower: 20,
            upper: 40,
        },
    ];
    println!("{:?}", intervals);

    intervals.sort_by_key(|i| i.upper); // Ordトレイトがないとsort_by_keyできない、
                                        // という話しだったが、できてしまっており、困っている。
    assert_eq!(
        intervals,
        vec![
            Interval { lower: 7, upper: 8 },
            Interval {
                lower: 10,
                upper: 20
            },
            Interval {
                lower: 20,
                upper: 40
            }
        ]
    );
    println!("{:?}", intervals);

    intervals.sort_by_key(|i| Reverse(i.lower));
    assert_eq!(
        intervals,
        vec![
            Interval {
                lower: 20,
                upper: 40
            },
            Interval {
                lower: 10,
                upper: 20
            },
            Interval { lower: 7, upper: 8 }
        ]
    );
    println!("{:?}", intervals);
}
