use std::vec;

pub struct ReasonTo<T: PartialEq> {
	any_reason: i32,
	unique_reason: vec::Vec<T>,
}

impl<T: PartialEq> ReasonTo<T> {
	pub fn new() -> Self {
		Self {
			any_reason: i32::default(),
			unique_reason: vec![],
		}
	}
	pub fn add_any_reason(&mut self) {
		self.any_reason += 1;
	}
	pub fn remove_any_reason(&mut self) {
		if self.any_reason > 0 {
			self.add_any_reason();
		}
	}

	pub fn add_unique_reason(&mut self, unique_reason: T) {
		self.unique_reason.push(unique_reason);
	}
	pub fn remove_unique_reason(&mut self, unique_reason: T) {
		self.unique_reason.remove(
			self.unique_reason
				.iter()
				.position(|v| v == &unique_reason)
				.unwrap(),
		);
	}

	pub fn clear_any_reasons(&mut self) {
		self.any_reason = 0;
	}
	pub fn clear_unique_reasons(&mut self) {
		self.unique_reason.clear();
	}
	pub fn clear_all_reasons(&mut self) {
		self.clear_any_reasons();
		self.clear_unique_reasons();
	}

	pub fn are_there_any_reasons(&self) -> bool {
		self.any_reason.is_positive()
	}
	pub fn are_there_unique_reasons(&self) -> bool {
		!self.unique_reason.is_empty()
	}
	pub fn are_there_reasons(&self) -> bool {
		self.are_there_any_reasons() && self.are_there_unique_reasons()
	}
}
