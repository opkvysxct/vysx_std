use std::vec;

#[derive(Default)]
pub struct ReasonToAny {
	any_reason: i32,
}

#[derive(Default)]
pub struct ReasonToUnique<T: PartialEq> {
	unique_reason: vec::Vec<T>,
}

#[derive(Default)]
pub struct ReasonToFull<T: PartialEq> {
	pub reason_to_any: ReasonToAny,
	pub reason_to_unique: ReasonToUnique<T>,
}

impl ReasonToAny {
	pub fn add_any_reason(&mut self) {
		self.any_reason += 1;
	}
	pub fn remove_any_reason(&mut self) {
		if self.any_reason.is_positive() {
			self.any_reason -= 1;
		}
	}
	pub fn clear_any_reasons(&mut self) {
		self.any_reason = 0;
	}
	pub fn are_there_any_reasons(&self) -> bool {
		self.any_reason.is_positive()
	}
}

impl<T: PartialEq> ReasonToUnique<T> {
	pub fn add_unique_reason(&mut self, unique_reason: T) {
		self.unique_reason.push(unique_reason);
	}
	pub fn remove_unique_reason(&mut self, unique_reason: T) {
		self.unique_reason.swap_remove(
			self.unique_reason
				.iter()
				.position(|reason| reason == &unique_reason)
				.unwrap(),
		);
	}

	pub fn clear_unique_reasons(&mut self) {
		self.unique_reason.clear();
	}
	pub fn are_there_unique_reasons(&self) -> bool {
		!self.unique_reason.is_empty()
	}
}

impl<T: PartialEq> ReasonToFull<T> {
	pub fn clear_all_reasons(&mut self) {
		self.reason_to_any.clear_any_reasons();
		self.reason_to_unique.clear_unique_reasons();
	}

	pub fn are_there_reasons(&self) -> bool {
		self.reason_to_any.are_there_any_reasons()
			&& self.reason_to_unique.are_there_unique_reasons()
	}
}
