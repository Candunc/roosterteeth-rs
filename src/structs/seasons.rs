use crate::structs::common::*;
use chrono::offset::FixedOffset;
use chrono::DateTime;
use serde::Deserialize;
use std::string::String;

#[derive(Debug, Deserialize)]
pub struct Root {
	pub data: Vec<Season>,
}

#[derive(Debug, Deserialize)]
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
	pub included: Included,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct EpisodesAvailable {
	pub sponsor: bool,
	pub member: bool,
	pub public: bool,
}

#[derive(Debug, Deserialize)]
pub struct Links {
	#[serde(rename = "self")]
	pub reference: String,
	pub episodes: String,
}

#[derive(Debug, Deserialize)]
pub struct Included {
	pub images: Vec<Image>,
}
