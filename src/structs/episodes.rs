use ::chrono::DateTime;
use ::chrono::offset::FixedOffset;
use ::serde::Deserialize;
use ::std::string::String;
use crate::structs::common::*;

#[derive(Deserialize)]
pub struct Root {
	pub data: Vec<Episode>,
	pub page: u16,
	pub per_page: u16,
	pub total_pages: u16,
	pub total_results: u32
}

/**
The Episodes struct is a strongly typed wrapper of the episodes endpoint. 
The full json format can be explored in your browser [here](https://svod-be.roosterteeth.com/api/v1/episodes),
or explore the object's fields below.

Most fields should have an example provided below, shown in the raw JSON format.
*/
#[derive(Deserialize)]
pub struct Episode {
	/**
	Kind is a renamed reference to the type of object the metadata is describing, in this context it will always be episode.
	```text
	"type": "episode"
	```
	*/
	#[serde(rename = "_index")]
	pub index: String,

	/** 
	Though stored as a vector, sort can be treated as an Option<u16>. It doesn't have a known meaning.
	
	```text
	"sort": [ 10001 ]
	```
	*/
	pub sort: Vec<u64>,

	/**
	The ID is the overall numeric identifier of the episode, ever increasing.
	
	```text 
	"id": 23242
	```
	*/
	pub id: u32,


	/**
	Index appears to be the timestamp the metadata was generated at.
	
	```text
	"_index": "episodes-production-en_20200108090014250"
	```
	*/
	#[serde(rename = "type")]
	pub kind: String,

	/**
	The UUID is a unique reference to an episode similar to it's slug.
	
	```text
	"uuid": "ffac28dc-464d-11e7-a302-065410f210c4"
	```
	*/
	pub uuid: String,
	pub attributes: Attributes,
	pub links: Links,
	pub canonical_links: CanonicalLinks,
	pub included: Included
}

#[derive(Deserialize)]
pub struct Attributes {
	pub title: String,
	pub slug: String,
	pub caption: String,
	pub number: u16,
	pub description: String,
	pub display_title: String,
	pub length: u32,

	pub advert_config: String,
	pub advertising: bool,
	pub ad_timestamps: String,

	pub public_golive_at: DateTime<FixedOffset>,
	pub sponsor_golive_at: DateTime<FixedOffset>,
	pub member_golive_at: DateTime<FixedOffset>,
	pub original_air_date: DateTime<FixedOffset>,

	pub channel_id: String,
	pub channel_slug: String,
	pub season_id: String,
	pub season_slug: String,
	pub season_number: u16,

	pub show_title: String,
	pub show_id: String,
	pub show_slug: String,
	pub is_sponsors_only: bool,
	pub member_tier_i: i8,
	pub sort_number: u32,
	pub genres: Vec<String>,

	pub is_live: bool,
	pub is_schedulable: bool,
	pub season_order: String,
	pub episode_order: String,
	pub downloadable: bool,
	pub blacklisted_countries: Vec<String>,
	pub upsell_next: bool,
}

#[derive(Deserialize)]
pub struct Links {
	#[serde(rename = "self")]
	pub reference: String,
	pub show: String,
	pub related_shows: String,
	pub channel: String,
	pub season: String,
	pub related: String,
	pub next: String,
	pub videos: String,
	pub products: String,
}

#[derive(Deserialize)]
pub struct CanonicalLinks {
	#[serde(rename = "self")]
	pub reference: String,

}

#[derive(Deserialize)]
pub struct Included {
	pub images: Vec<Image>,
	pub tags: Vec<Tag>,
	pub cast_members: Vec<String>,
}

#[derive(Deserialize)]
pub struct Tag {
	pub id: String,
	pub uuid: String,
	#[serde(rename = "type")]
	pub kind: String,

	pub attributes: TagAttribute,

//	pub links: bool,
//	pub included: bool,
}

#[derive(Deserialize)]
pub struct TagAttribute {
	pub tag: String,
	pub slug: String,
}
