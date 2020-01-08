use ::serde::Deserialize;

#[derive(Deserialize)]
pub struct Image {
	pub id: u32,
	pub uuid: String,
	#[serde(rename = "type")]
	pub kind: String,
	pub attributes: ImageAttributes,

	// Not entirely sure what these are for - they are blank in the api.
//	pub links: bool,
//	pub included: bool,
}

#[derive(Deserialize)]
pub struct ImageAttributes {
	pub thumb: String,
	pub small: String,
	pub medium: String,
	pub large: String,
	pub orientation: String,
	pub image_type: String,
}
