use ::reqwest::blocking::Client;
use ::std::error;
use ::std::fmt;

use crate::structs::*;

const USER_AGENT: &'static str = "Mozilla/5.0 roosterteeth-rs/0.1.0 reqwest/0.10.0";

const API_URL: &'static str = "https://svod-be.roosterteeth.com/api/v1";

type Result<T> = std::result::Result<T, VideoUnavailable>;

#[derive(Debug, Clone)]
pub struct VideoUnavailable;

impl fmt::Display for VideoUnavailable {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Video is not (yet?) avaliable for non-sponsors.")
	}
}

impl error::Error for VideoUnavailable {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		None
	}
}

fn append_channel_and_order<'a>(url: &mut String, channel: Option<&'a str>, order: Option<&'a str>) {
	if channel != None {
		url.push_str("&channel_id=");
		url.push_str(channel.unwrap());
	}

	url.push_str("&order=");
	if order == None {
		url.push_str("desc");
	} else {
		url.push_str(order.unwrap());
	}
}

/// The requests client is split up into two classes of functions, list_* which does not require arguments,
/// and get_*, which requires the slug to retrieve further information.
pub struct Requests {
	client: Client
}

impl Requests {
	/// This creates the [Reqwest](https://docs.rs/reqwest/latest/reqwest/) client used to make the rest of our calls.
	pub fn new() -> Self {
		Requests {
			client: Client::builder()
				.user_agent(USER_AGENT)
				.build()
				.expect("Unable to build the reqwest client")
		}
	}

	/// Returns a list of avaliable RoosterTeeth channels that can be used by the API.
	pub fn list_channels(&self) -> Vec<channels::Channel> {
		let url = format!("{}/channels", API_URL);

		let result: channels::Root = self.client.get(&url)
			.send().unwrap().json().unwrap();

		result.data
	}

	/// This returns up to 100 episodes from the RoosterTeeth API.
	/// Channels are specified by their slug, and order is either 'asc' for ascending or 'dec' for descending.
	pub fn list_episodes<'a>(&self, page: u16, channel: Option<&'a str>, order: Option<&'a str>) -> Vec<episodes::Episode> {
		let mut url = format!("{}/episodes?per_page=100", API_URL);
		
		append_channel_and_order(&mut url, channel, order);

		url = format!("{}&page={}",url,page);

		let result: episodes::Root = self.client.get(&url)
			.send().unwrap().json().unwrap();

		result.data
	}

	pub fn list_series<'a>(&self, channel: Option<&'a str>, order: Option<&'a str>) -> Vec<series::Series> {
		let mut url = format!("{}/shows?per_page=1000", API_URL);

		append_channel_and_order(&mut url, channel, order);

		url.push_str("&page=1");

		let result: series::Root = self.client.get(&url)
			.send().unwrap().json().unwrap();

		result.data
	}

	/// Gets all season information from a specific series from its slug.
	pub fn get_seasons<'a>(&self, slug: &'a str, order: Option<&'a str>) -> Vec<seasons::Season> {
		let mut url = format!("{}/shows/{}/seasons?order=", API_URL, slug);

		if order == None {
			url.push_str("desc");
		} else {
			url.push_str(order.unwrap());
		}

		let result: seasons::Root = self.client.get(&url)
			.send().unwrap().json().unwrap();

		result.data
	}

	/// Gets the episodes belonging to a specific season by its slug.
	pub fn get_season_episodes<'a>(&self, slug: &'a str) -> Vec<episodes::Episode> {
		// https://svod-be.roosterteeth.com/api/v1/seasons/red-vs-blue-season-1/episodes?order=asc
		let url = format!("{}/seasons/{}/episodes?order=asc",API_URL, slug);

		let result: episodes::Root = self.client.get(&url)
			.send().unwrap().json().unwrap();

		result.data
	}

	/// Gets a specific series information from its slug.
	/// This returns an identical result to those of list_series()
	pub fn get_series<'a>(&self, slug: &'a str) -> series::Series {
		let url = format!("{}/shows/{}",API_URL, slug);

		let mut result: series::Root = self.client.get(&url)
			.send().unwrap().json().unwrap();

		result.data.remove(0)
	}

	/// Gets an episodes viewing information from its slug. 
	/// Please note that this can result in an error if we don't have the permission to
	/// view that video.
	pub fn get_video<'a>(&self, slug: &'a str) -> Result<videos::Video> {
		let url = format!("{}/watch/{}/videos", API_URL, slug);

		let response = self.client.get(&url).send().unwrap();

		if response.status().is_success() {
			let mut result: videos::Root = response.json().unwrap();

			Ok(result.data.remove(0))

		} else {
			Err(VideoUnavailable)
		}
	}
}
