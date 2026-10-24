use std::ops::{Add, Mul, Sub};

/// A pair of bounds, `min` and `max`, that can be interpolated between.
///
/// The default value uses `T::default()` for both bounds and requires
/// `T: Default`.
#[derive(Default)]
pub struct MinMax<T> {
	min: T,
	max: T,
}

impl<T> MinMax<T> {
	/// Creates a new `MinMax` from the given bounds.
	///
	/// No check is made that `min <= max`.
	pub fn new(min: T, max: T) -> Self {
		Self { min, max }
	}
}

impl<T> MinMax<T> {
	/// Linearly interpolates from `min` to `max`.
	///
	/// Returns `min + (max - min) * by`: `min` at `by = 0` and `max` at
	/// `by = 1`. `by` is not clamped, so values outside `0..=1` extrapolate.
	pub fn lerp<S>(&self, by: S) -> T
	where
		T: Copy + Add<Output = T> + Sub<Output = T> + Mul<S, Output = T>,
	{
		self.min + (self.max - self.min) * by
	}

	/// Linearly interpolates from `max` to `min`.
	///
	/// Returns `max + (min - max) * by`: `max` at `by = 0` and `min` at
	/// `by = 1`. Equivalent to `lerp(1 - by)`. `by` is not clamped.
	pub fn reverse_lerp<S>(&self, by: S) -> T
	where
		T: Copy + Add<Output = T> + Sub<Output = T> + Mul<S, Output = T>,
	{
		self.max + (self.min - self.max) * by
	}
}
