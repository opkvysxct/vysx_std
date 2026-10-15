use std::ops::{Add, Mul, Sub};

#[derive(Default)]
pub struct MinMax<T> {
	min: T,
	max: T,
}

impl<T> MinMax<T> {
	pub fn new(min: T, max: T) -> Self {
		Self { min, max }
	}
}

impl<T> MinMax<T> {
	pub fn lerp<S>(&self, by: S) -> T
	where
		T: Copy + Add<Output = T> + Sub<Output = T> + Mul<S, Output = T>,
	{
		self.min + (self.max - self.min) * by
	}

	pub fn reverse_lerp<S>(&self, by: S) -> T
	where
		T: Copy + Add<Output = T> + Sub<Output = T> + Mul<S, Output = T>,
	{
		self.max + (self.min - self.max) * by
	}

}
