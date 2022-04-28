use std::fmt;

pub struct ZeusError {
	pub facility: &'static str,
	pub data: String,
}

#[allow(dead_code)]
impl ZeusError {
	pub fn new(facility: &'static str, data: String) -> Self {
		Self {
			facility: facility,
			data: data,
		}
	}
}

impl fmt::Display for ZeusError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}: {}", self.facility, self.data)
	}
}

impl fmt::Debug for ZeusError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}: {}", self.facility, self.data)
	}
}

impl std::error::Error for ZeusError {}
