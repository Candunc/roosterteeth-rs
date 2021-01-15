use crate::structs::common::*;
use chrono::offset::FixedOffset;
use chrono::DateTime;
use serde::Deserialize;
use std::string::String;

#[derive(Debug, Deserialize)]
pub struct Root {
	pub data: Vec<Episode>,
	pub page: Option<u16>,
	pub per_page: Option<u16>,
	pub total_pages: Option<u16>,
	pub total_results: Option<u32>,
}

/**
The Episodes struct is a strongly typed wrapper of the episodes endpoint.
The full json format can be explored in your browser [here](https://svod-be.roosterteeth.com/api/v1/episodes),
or explore the object's fields below.

Most fields should have an example provided below, shown in the raw JSON format.
*/
#[derive(Debug, Deserialize)]
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
	pub sort: Option<Vec<u64>>,

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
	pub included: Included,
}

/// Most elements here will be fairly self-documenting.
#[derive(Debug, Deserialize)]
pub struct Attributes {
	/**
	```text
	"title": "Episode 1: Why Are We Here?"
	```
	*/
	pub title: String,

	/**
	The url suffix of the episode, will be unique.

	```text
	"slug": "red-vs-blue-season-1-episode-1"
	```
	*/
	pub slug: String,

	/**
	```text
	"caption": "Why Are We Here?"
	```
	*/
	pub caption: String,

	/**
	```text
	"number": 1
	```
	*/
	pub number: u16,

	/**
	```text
	"description": "The first episode of Red... why are we here"
	```
	*/
	pub description: String,

	/**
	```text
	"display_title": "S1:E1 - Episode 1: Why Are We Here?"
	```
	*/
	pub display_title: String,

	/**
	The length of the episode in seconds

	```text
	"length": 256
	```
	*/
	pub length: u32,

	pub advert_config: String,
	pub advertising: bool,

	/**
	Comma seperated timestamps for valid ad placements

	```text
	"ad_timestamps": "60.00,120.00,180.00"
	```
	*/
	pub ad_timestamps: Option<String>,

	/**
	Technically the API returns a ISO 8601 compliant datetime, however the API
	uses [Chrono](https://docs.rs/chrono/latest/chrono/) to make interacting
	with the datetime more convenient.
	*/
	pub public_golive_at: DateTime<FixedOffset>,
	pub sponsor_golive_at: DateTime<FixedOffset>,
	pub member_golive_at: DateTime<FixedOffset>,
	pub original_air_date: DateTime<FixedOffset>,

	/**
	```text
	"channel_id": "92b6bb21-91d2-4b1b-bf95-3268fa0d9939"
	``
	*/
	pub channel_id: String,

	/**
	```text
	"channel_slug": "rooster-teeth"
	```
	*/
	pub channel_slug: String,

	/**
	```text
	"season_id": "ffa11de8-464d-11e7-a302-065410f210c4"
	```
	*/
	pub season_id: String,

	/**
	```text
	"season_slug": "red-vs-blue-season-1"
	```
	*/
	pub season_slug: String,

	/**
	```text
	"season_number": 1
	```
	*/
	pub season_number: u16,

	/**
	```text
	"show_title": "Red vs. Blue"
	```
	*/
	pub show_title: String,

	/**
	```text
	"show_id": "ff925ff9-464d-11e7-a302-065410f210c4"
	```
	*/
	pub show_id: String,

	/**
	```text
	"show_slug": "red-vs-blue"
	```
	*/
	pub show_slug: String,

	/**
	```text
	"is_sponsors_only": false
	```
	*/
	pub is_sponsors_only: bool,

	/**
	```text
	"member_tier_1": -1
	```
	*/
	pub member_tier_i: i8,

	/**
	```text
	"sort_number": 10001
	```
	*/
	pub sort_number: u32,

	/**
	```text
	genres: ["Action Packed", "Full of Laughs", "Games Reimagined", "Classic Rooster Teeth", "Rooster Teeth Originals"]
	```
	**/
	pub genres: Vec<String>,

	/**
	```text
	"is_live": true
	```
	*/
	pub is_live: bool,

	/**
	This endpoint is likely used to create the video list for RT-TV.

	```text
	"is_schedulable": true
	```
	*/
	pub is_schedulable: bool,

	/**
	Default sorting, will either be "asc" or "desc"

	```text
	"season_order": "desc"
	```
	*/
	pub season_order: String,

	/**
	```text
	"episode_order": "asc"
	```
	*/
	pub episode_order: String,

	/**
	Determines whether the video endpoint will have a links.download element.

	```text
	"downloadable": true
	```
	*/
	pub downloadable: bool,

	/**
	```text
	"blacklisted_countries": []
	```
	*/
	pub blacklisted_countries: Vec<String>,

	/**
	```text
	"upsell_next": false
	```
	*/
	pub upsell_next: bool,
}

/**
Links is a reference to the various other API endpoints relevant to this entry.

TODO: Implement these as object-oriented functions!
*/
#[derive(Debug, Deserialize)]
pub struct Links {
	#[serde(rename = "self")]
	pub reference: String,
	pub show: String,
	pub related_shows: String,
	pub channel: String,
	pub season: String,
	pub next: String,
	pub videos: String,
	pub products: String,
}

/**
Canonical Links references the public webpage this data describes, rather than the API endpoint
*/
#[derive(Debug, Deserialize)]
pub struct CanonicalLinks {
	/**
	```text
	"self": "/watch/red-vs-blue-season-1-episode-1"
	```
	*/
	#[serde(rename = "self")]
	pub reference: String,

	/**
	```text
	"show": "/series/red-vs-blue"
	```
	*/
	pub show: String,
}

#[derive(Debug, Deserialize)]
pub struct Included {
	pub images: Vec<Image>,
	pub tags: Vec<Tag>,
	pub cast_members: Vec<CastMember>,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
	pub id: String,
	pub uuid: String,
	#[serde(rename = "type")]
	pub kind: String,

	pub attributes: TagAttribute,
	//	pub links: bool,
	//	pub included: bool,
}

#[derive(Debug, Deserialize)]
pub struct TagAttribute {
	pub tag: String,
	pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct CastMember {
	pub id: String,
	pub uuid: String,
	#[serde(rename = "type")]
	pub kind: String,
	pub attributes: CastMemberAttributes,
	//	pub links: bool,
	//	pub included: bool,
}

#[derive(Debug, Deserialize)]
pub struct CastMemberAttributes {
	pub name: String,
}
