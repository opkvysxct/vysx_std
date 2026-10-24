/// Tracks anonymous reasons as a simple counter.
///
/// Each [`add`](Self::add) should be paired with a [`remove`](Self::remove).
/// [`any`](Self::any) is `true` while at least one reason remains.
#[derive(Default)]
pub struct ReasonToAny {
	count: u32,
}

/// Tracks a set of distinct, identifiable reasons.
///
/// A reason can only be present once. Order is not preserved.
#[derive(Default)]
pub struct ReasonToUnique<T> {
	reasons: Vec<T>,
}

/// Combines [`ReasonToAny`] and [`ReasonToUnique`].
///
/// Use `unique` for reasons that can be identified, and `any` for ones that
/// can't.
#[derive(Default)]
pub struct ReasonToFull<T> {
	pub any: ReasonToAny,
	pub unique: ReasonToUnique<T>,
}

impl ReasonToAny {
	/// Adds a reason.
	pub fn add(&mut self) {
		self.count += 1;
	}

	/// Removes a reason. Does nothing if there are none.
	pub fn remove(&mut self) {
		self.count = self.count.saturating_sub(1);
	}

	/// Removes all reasons.
	pub fn clear(&mut self) {
		self.count = 0;
	}

	/// Returns `true` if at least one reason is present.
	pub fn any(&self) -> bool {
		self.count != 0
	}
}

impl<T: PartialEq> ReasonToUnique<T> {
	/// Adds `reason` if it isn't already present.
	///
	/// Returns `true` if it was added, `false` if it already existed.
	pub fn add(&mut self, reason: T) -> bool {
		if self.reasons.contains(&reason) {
			return false;
		}
		self.reasons.push(reason);
		true
	}

	/// Removes `reason` if present.
	///
	/// Returns `true` if it was removed, `false` if it wasn't found.
	pub fn remove(&mut self, reason: &T) -> bool {
		match self.reasons.iter().position(|r| r == reason) {
			Some(pos) => {
				self.reasons.swap_remove(pos);
				true
			}
			None => false,
		}
	}

	/// Removes all reasons.
	pub fn clear(&mut self) {
		self.reasons.clear();
	}

	/// Returns `true` if at least one reason is present.
	pub fn any(&self) -> bool {
		!self.reasons.is_empty()
	}
}

impl<T: PartialEq> ReasonToFull<T> {
	/// Removes all reasons, both anonymous and unique.
	pub fn clear(&mut self) {
		self.any.clear();
		self.unique.clear();
	}

	/// Returns `true` if either an anonymous or a unique reason is present.
	pub fn any_reasons(&self) -> bool {
		self.any.any() || self.unique.any()
	}
}
