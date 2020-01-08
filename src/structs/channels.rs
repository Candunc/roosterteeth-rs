use ::serde::Deserialize;
use ::std::string::String;
use crate::structs::common::*;

#[derive(Deserialize)]
pub struct Root {
	pub data: Vec<Channel>,
	pub page: u16,
	pub per_page: u16,
	pub total_pages: u16,
	pub total_results: u32
}

#[derive(Deserialize)]
pub struct Channel {
	#[serde(rename = "_index")]
	pub index: String,

	#[serde(rename = "type")]
	pub kind: String,

	pub sort: Vec<u16>,
	pub id: u16,
	pub uuid: String,

	pub attributes: Attributes,
	pub included: Included,
	pub links: Links,
}

#[derive(Deserialize)]
pub struct Attributes {
	pub name: String,
	pub importance: u16,
	pub slug: String,
	pub brand_color: String,
}

#[derive(Deserialize)]
pub struct Included {
	pub images: Vec<Image>,
}

#[derive(Deserialize)]
pub struct Links {
	#[serde(rename = "self")]
	pub reference: String,
	pub shows: String,
	pub movies: String,
	pub product_collections: String,
	pub featured_items: String,
	pub episodes: String,
	pub livestreams: String,
}
