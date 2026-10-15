#[derive(Default)]
pub struct ReasonToAny {
	count: u32,
}

#[derive(Default)]
pub struct ReasonToUnique<T> {
	reasons: Vec<T>,
}

#[derive(Default)]
pub struct ReasonToFull<T> {
	pub any: ReasonToAny,
	pub unique: ReasonToUnique<T>,
}

impl ReasonToAny {
	pub fn add(&mut self) {
		self.count += 1;
	}
	pub fn remove(&mut self) {
		self.count = self.count.saturating_sub(1);
	}
	pub fn clear(&mut self) {
		self.count = 0;
	}
	pub fn any(&self) -> bool {
		self.count != 0
	}
}

impl<T: PartialEq> ReasonToUnique<T> {
	pub fn add(&mut self, reason: T) -> bool {
		if self.reasons.contains(&reason) {
			return false;
		}
		self.reasons.push(reason);
		true
	}

	pub fn remove(&mut self, reason: &T) -> bool {
		match self.reasons.iter().position(|r| r == reason) {
			Some(pos) => {
				self.reasons.swap_remove(pos);
				true
			}
			None => false,
		}
	}

	pub fn clear(&mut self) {
		self.reasons.clear();
	}
	pub fn any(&self) -> bool {
		!self.reasons.is_empty()
	}
}

impl<T: PartialEq> ReasonToFull<T> {
	pub fn clear(&mut self) {
		self.any.clear();
		self.unique.clear();
	}

	pub fn any_reasons(&self) -> bool {
		self.any.any() || self.unique.any()
	}
}
