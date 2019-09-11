//! Useful common math operations for doing art.
use std::ops::{RangeFrom, RangeInclusive, RangeToInclusive};

/// Represent types that can be restricted by a given range type.
///
/// This would've been called `Clamp`, except that there's a standard library
/// method called `clamp`.
pub trait Restrict<RangeType> {
    /// Restrict a value into a given range.
    ///
    /// If a value is below the minimum bound, it should be clamped to that
    /// value, and if it's above its max value it should be clamped to that.
    /// This is only provided for inclusive ranges, since the behavior for
    /// exclusive ranges of some types are less immediately clear.
    fn restrict(self, range: RangeType) -> Self;
}

impl<T> Restrict<RangeInclusive<T>> for T
where
    T: PartialOrd,
{
    fn restrict(self, range: RangeInclusive<T>) -> T {
        let (start, end) = range.into_inner();
        if self > end {
            return end;
        }
        if self < start {
            return start;
        }
        self
    }
}

impl<T> Restrict<RangeToInclusive<T>> for T
where
    T: PartialOrd,
{
    fn restrict(self, range: RangeToInclusive<T>) -> T {
        if self > range.end {
            return range.end;
        }
        self
    }
}

impl<T> Restrict<RangeFrom<T>> for T
where
    T: PartialOrd,
{
    fn restrict(self, range: RangeFrom<T>) -> T {
        if self < range.start {
            return range.start;
        }
        self
    }
}
