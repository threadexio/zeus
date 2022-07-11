use serde::Deserialize;

pub const IMAGE_FMT: &'static str = "{\"name\":\"{{.Repository}}\"}";

#[derive(Deserialize)]
pub struct Image {
	pub id: String,
	pub name: String,
}

pub const CONTAINER_FMT: &'static str = "{\"name\":\"{{.Names}}\"}";

#[derive(Deserialize)]
pub struct Container {
	pub name: String,
}
