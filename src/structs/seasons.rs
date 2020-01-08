use ::chrono::DateTime;
use ::chrono::offset::FixedOffset;
use ::serde::Deserialize;
use ::std::string::String;
use crate::structs::common::*;

#[derive(Deserialize)]
pub struct Root {
	pub data: Vec<Season>
}

#[derive(Deserialize)]
pub struct Season {
	#[serde(rename = "_index")]
	pub index: String,
	pub sort: Vec<u16>,
	pub id: u32,
	#[serde(rename = "type")]
	pub kind: String,
	pub uuid: String,
	pub attributes: Attributes,
	pub links: Links,
	pub included: Included
}

#[derive(Deserialize)]
pub struct Attributes {
	pub title: String,
	pub description: String,
	pub slug: String,
	pub number: u16,
	pub show_id: String,
	pub show_slug: String,
	pub episodes_available: EpisodesAvailable,
	pub published_at: DateTime<FixedOffset>,
}

#[derive(Deserialize)]
pub struct EpisodesAvailable {
	pub sponsor: bool,
	pub member: bool,
	pub public: bool,
}

#[derive(Deserialize)]
pub struct Links {
	#[serde(rename = "self")]
	pub reference: String,
	pub episodes: String,
}

#[derive(Deserialize)]
pub struct Included {
	pub images: Vec<Image>,
}
