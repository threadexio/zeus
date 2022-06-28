use zeus::machine::{IImage, IMachine};

use serde::Deserialize;

pub const IMAGE_FMT: &'static str =
	"{\"id\":\"{{.ID}}\",\"name\":\"{{.Repository}}\"}";

#[derive(Deserialize)]
pub struct Image {
	pub id: String,
	pub name: String,
}

impl IImage for Image {
	fn id(&self) -> &str {
		&self.id
	}

	fn name(&self) -> &str {
		&self.name
	}
}

pub const CONTAINER_FMT: &'static str =
	"{\"id\":\"{{.ID}}\",\"name\":\"{{.Names}}\",\"image\":\"{{.Image}}\"}";

#[derive(Deserialize)]
pub struct Container {
	pub id: String,
	pub name: String,
	pub image: String,
}

impl IMachine for Container {
	fn id(&self) -> &str {
		&self.id
	}

	fn name(&self) -> &str {
		&self.name
	}

	fn image(&self) -> &str {
		&self.image
	}
}
