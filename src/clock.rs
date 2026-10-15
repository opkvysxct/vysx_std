use std::ops::{AddAssign, RemAssign};

#[derive(Default)]
pub struct Clock<T> {
	elapsed: T,
}

pub trait Tau {
	const TAU: Self;
}

impl Tau for f32 {
	const TAU: f32 = std::f32::consts::TAU;
}

impl Tau for f64 {
	const TAU: f64 = std::f64::consts::TAU;
}

impl<T> Clock<T>
where
	T: AddAssign + RemAssign + Tau + Copy,
{
	pub fn increase_tau(&mut self, by: T) {
		self.elapsed += by;
		self.elapsed %= T::TAU;
	}
	pub fn increase(&mut self, by: T) {
		self.elapsed += by;
	}
	pub fn get(&self) -> T {
		self.elapsed
	}
	pub fn set(&mut self, to: T) {
		self.elapsed = to;
	}
}
