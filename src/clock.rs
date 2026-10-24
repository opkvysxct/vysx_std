use std::ops::{AddAssign, RemAssign};

/// An accumulator for elapsed time or phase, typically measured in radians.
///
/// `Clock` stores a single value of type `T` (usually `f32` or `f64`) and offers
/// two ways to advance it: [`increase`](Clock::increase), which accumulates
/// without bound, and [`increase_tau`](Clock::increase_tau), which wraps the
/// value into one full turn (`0..TAU`). This makes it handy for driving periodic
/// functions such as `sin` and `cos` in animations or oscillators without the
/// value growing indefinitely and losing floating-point precision.
///
/// A new clock created with [`Default::default`] starts at zero. This requires
/// `T: Default`.
#[derive(Default)]
pub struct Clock<T> {
	elapsed: T,
}

/// A type that has a constant representing a full turn (`τ = 2π`).
///
/// This allows [`Clock`] to wrap its value generically over different
/// floating-point types. It is implemented for [`f32`] and [`f64`].
pub trait Tau {
	/// The full circle constant `τ`, equal to `2π` (≈ 6.28318).
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
	/// Advances the clock by `by` and wraps the result into one full turn.
	pub fn increase_tau(&mut self, by: T) {
		self.elapsed += by;
		self.elapsed %= T::TAU;
	}

	/// Advances the clock by `by` without wrapping.
	///
	/// The value grows without bound. Use [`increase_tau`](Clock::increase_tau)
	/// instead if you want the value to stay within one full turn.
	pub fn increase(&mut self, by: T) {
		self.elapsed += by;
	}

	/// Returns the current elapsed value.
	pub fn get(&self) -> T {
		self.elapsed
	}

	/// Overwrites the current elapsed value with `to`.
	///
	/// The value is stored as given and is not wrapped, even if it is
	/// greater than `T::TAU`.
	pub fn set(&mut self, to: T) {
		self.elapsed = to;
	}
}
