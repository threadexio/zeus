use serde::Deserialize;

pub const IMAGE_FMT: &str = "{\"name\":\"{{.Repository}}:{{.Tag}}\"}";

#[derive(Deserialize)]
pub struct Image {
	pub name: String,
}

pub const CONTAINER_FMT: &str = "{\"name\":\"{{.Names}}\"}";

#[derive(Deserialize)]
pub struct Container {
	pub name: String,
}
