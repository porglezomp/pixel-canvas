//! Useful common math operations for doing art.
use std::ops::{Add, Div, Mul, Range, RangeFrom, RangeInclusive, RangeToInclusive, Sub};

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

/// Represents a type that can be mapped between two ranges.
pub trait Remap where Self: Sized {
    /// Remap a value from one range to another. A value outside the bounds of
    /// one range will be similarly outside the bounds of the other.
    /// ```rust
    /// # use pixel_canvas::prelude::*;
    /// assert_eq!(5.remap(-10..10, -100..100), 50);
    /// assert_eq!(0.5.remap(0.0..1.0, -1.0..1.0), 0.0);
    /// ```
    fn remap(self, from: Range<Self>, onto: Range<Self>) -> Self;
}

impl<T> Remap for T
where
    T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy,
{
    fn remap(self, from: Range<Self>, onto: Range<Self>) -> Self {
        let from_size = from.end - from.start;
        let onto_size = onto.end - onto.start;
        ((self - from.start) * onto_size / from_size) + onto.start
    }
}
