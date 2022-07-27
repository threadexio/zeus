use serde::Deserialize;

pub const IMAGE_FMT: &'static str =
	"{\"name\":\"{{.Repository}}:{{.Tag}}\"}";

#[derive(Deserialize)]
pub struct Image {
	pub name: String,
}

pub const CONTAINER_FMT: &'static str = "{\"name\":\"{{.Names}}\"}";

#[derive(Deserialize)]
pub struct Container {
	pub name: String,
}
