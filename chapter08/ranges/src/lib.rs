use std::ops::Range;

/// Return true if two ranges overlap.
///     
///     assert_eq!(ranges::overlap(0..7, 3..10), true);
///     # let r = 0..7;
///     # assert_eq!(ranges::overlap(r, 3..10), true);
///     assert_eq!(ranges::overlap(1..5, 101..105), false);
///
/// If either range is empty, they don't count as overlapping
///
/// ```no_run
///assert_eq!(ranges::overlap(0..0, 0..10), false);
///```
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end && r1.end > r2.start && r2.end > r1.start
}
