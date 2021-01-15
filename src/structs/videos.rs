use chrono::offset::FixedOffset;
use chrono::DateTime;
use serde::Deserialize;
use std::string::String;

#[derive(Debug, Deserialize)]
pub struct Root {
	pub data: Vec<Video>,
}

/**
The Video struct is a strongly typed wrapper of an episode's video endpoint.
The full json format can be explored in your browser [here](https://svod-be.roosterteeth.com/api/v1/watch/red-vs-blue-season-2-episode-22/videos),
or explore the object's fields below.
*/
#[derive(Debug, Deserialize)]
pub struct Video {
	#[serde(rename = "_index")]
	pub index: String,
	#[serde(rename = "_score")]
	pub score: f32,
	pub id: u32,
	#[serde(rename = "type")]
	pub kind: String,
	pub uuid: String,
	pub attributes: Attributes,
	pub links: Links,
	//	pub included: Included
}

#[derive(Debug, Deserialize)]
pub struct Attributes {
	pub url: String,
	pub content_id: u32,
	pub content_slug: String,
	pub content_uuid: String,

	pub public_golive_at: DateTime<FixedOffset>,
	pub sponsor_golive_at: DateTime<FixedOffset>,
	pub member_golive_at: DateTime<FixedOffset>,

	//	pub frame_sizes: Vec<String>,
	//	pub intro_starts_at: Option<()>,
	//	pub intro_ends_at: Option<()>,
	pub media_type: String,
	pub member_tier: String,
	//	pub bandwidth: bool,
	pub embed: bool,
	pub is_sponsors_only: bool,
	pub image_pattern_url: Option<String>,
	pub bif_url: Option<String>,
	pub ad_config: Option<AdConfig>,
}

#[derive(Debug, Deserialize)]
pub struct AdConfig {
	pub nw: String,
	pub caid: String,
	pub afid: String,
	pub prof: String,
	pub ad_timestamps: Option<Vec<u32>>,
	pub preroll: Vec<String>,
	pub midroll: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Links {
	#[serde(rename = "self")]
	pub reference: String,
	pub content: String,
	pub download: String,
}
