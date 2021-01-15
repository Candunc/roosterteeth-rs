use crate::structs::common::*;
use chrono::offset::FixedOffset;
use chrono::DateTime;
use serde::Deserialize;
use std::string::String;

#[derive(Debug, Deserialize)]
pub struct Root {
	pub data: Vec<Series>,

	// This only is filled when we use the list function, get doesn't return these values.
	pub page: Option<u16>,
	pub per_page: Option<u16>,
	pub total_pages: Option<u16>,
	pub total_results: Option<u32>,
}

/**
The Series struct is a strongly typed wrapper of the show endpoint.
The full json format can be explored in your browser [here](https://svod-be.roosterteeth.com/api/v1/shows?per_page=1000),
or explore the object's fields below.
*/
#[derive(Debug, Deserialize)]
pub struct Series {
	#[serde(rename = "_index")]
	pub index: String,
	pub sort: Vec<u64>,
	pub id: u32,
	#[serde(rename = "type")]
	pub kind: String,
	pub uuid: String,
	pub attributes: Attributes,
	pub links: Links,
	pub canonical_links: CanonicalLinks,
	pub included: Included,
}

#[derive(Debug, Deserialize)]
pub struct Attributes {
	pub title: String,
	pub slug: String,
	pub genres: Vec<String>,
	pub is_sponsors_only: bool,

	pub updated_at: DateTime<FixedOffset>,
	pub published_at: DateTime<FixedOffset>,
	pub last_episode_golive_at: DateTime<FixedOffset>,

	pub summary: String,
	pub category: String,
	pub channel_id: String,
	pub channel_slug: String,
	pub season_count: u16,
	pub episode_count: u32,

	pub season_order: String,
	pub episode_order: String,

	pub blacklisted_countries: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Links {
	#[serde(rename = "self")]
	pub reference: String,
	pub seasons: String,
	pub bonus_features: String,
	pub related: String,
	pub product_collections: String,
	pub latest_episode: String,
	pub s1e1: String,
	pub rich_card_reference_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CanonicalLinks {
	#[serde(rename = "self")]
	pub reference: String,
	pub s1e1: String,
}

#[derive(Debug, Deserialize)]
pub struct Included {
	pub images: Vec<Image>,
}
