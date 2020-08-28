use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::error;
use std::fmt;

use crate::structs::*;

const USER_AGENT: &'static str = "Mozilla/5.0 roosterteeth-rs/0.1.0 reqwest/0.10.4";

const LOGIN_URL: &'static str = "https://auth.roosterteeth.com/oauth/token";
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

fn append_channel_and_order<'a>(
	url: &mut String,
	channel: Option<&'a str>,
	order: Option<&'a str>,
) {
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

#[derive(Debug, Deserialize)]
struct Token {
	access_token: String,
	token_type: String,
	expires_in: u32,
	refresh_token: String,
	scope: String,
	created_at: u32,
	user_id: u32,
	uuid: String,
}

pub enum Credential<'a> {
	Anonymous,
	Login(&'a str, &'a str),
}

/// The requests client is split up into two classes of functions, list_* which does not require arguments,
/// and get_*, which requires the slug to retrieve further information.
pub struct Requests {
	client: Client,

	// Used for storing bearer: <token> in case of authenticated download
	headers: HeaderMap,
}

impl Requests {
	/// This creates the [Reqwest](https://docs.rs/reqwest/latest/reqwest/) client used to make the rest of our calls.
	pub fn new(credential: Credential) -> Self {
		let client = Client::builder()
			.user_agent(USER_AGENT)
			.build()
			.expect("Unable to build the reqwest client");

		let login: Option<(&str, &str)> = match credential {
			Credential::Anonymous => None,
			Credential::Login(u, p) => Some((u, p)),
		};

		let mut headers = HeaderMap::new();
		if login.is_some() {
			let (user, pass) = login.unwrap();

			let body = format!(
				"{{\"client_id\":\"4338d2b4bdc8db1239360f28e72f0d9ddb1fd01e7a38fbb07b4b1f4ba4564cc5\",\"grant_type\":\"password\",\"password\":\"{}\",\"scope\":\"user public\",\"username\":\"{}\"}}",
				pass,
				user,
			);

			let token: Token = client
				.post(LOGIN_URL)
				.body(body)
				.send()
				.unwrap()
				.json()
				.unwrap();

			headers.insert(
				"authorization",
				format!("Bearer {}", token.access_token).parse().unwrap(),
			);
		}

		Requests {
			client: client,
			headers: headers,
		}
	}

	/// Returns a list of avaliable RoosterTeeth channels that can be used by the API.
	pub fn list_channels(&self) -> Vec<channels::Channel> {
		let url = format!("{}/channels", API_URL);

		let result: channels::Root = self.client.get(&url).send().unwrap().json().unwrap();

		result.data
	}

	/// This returns up to 100 episodes from the RoosterTeeth API.
	/// Channels are specified by their slug, and order is either 'asc' for ascending or 'dec' for descending.
	pub fn list_episodes<'a>(
		&self,
		page: u16,
		channel: Option<&'a str>,
		order: Option<&'a str>,
	) -> Vec<episodes::Episode> {
		let mut url = format!("{}/episodes?per_page=100", API_URL);

		append_channel_and_order(&mut url, channel, order);

		url = format!("{}&page={}", url, page);

		let result: episodes::Root = self.client.get(&url).send().unwrap().json().unwrap();

		result.data
	}

	pub fn list_series<'a>(
		&self,
		channel: Option<&'a str>,
		order: Option<&'a str>,
	) -> Vec<series::Series> {
		let mut url = format!("{}/shows?per_page=1000", API_URL);

		append_channel_and_order(&mut url, channel, order);

		url.push_str("&page=1");

		let result: series::Root = self.client.get(&url).send().unwrap().json().unwrap();

		result.data
	}

	/// Gets all season information from a specific series from its slug.
	pub fn get_seasons<'a>(&self, slug: &'a str, order: Option<&'a str>) -> Vec<seasons::Season> {
		let url = format!(
			"{}/shows/{}/seasons?order={}",
			API_URL,
			slug,
			order.unwrap_or("desc"),
		);

		let result: seasons::Root = self.client.get(&url).send().unwrap().json().unwrap();

		result.data
	}

	/// Gets the episodes belonging to a specific season by its slug.
	pub fn get_season_episodes<'a>(
		&self,
		slug: &'a str,
		order: Option<&'a str>,
	) -> Vec<episodes::Episode> {
		let url = format!(
			"{}/seasons/{}/episodes?order={}&per_page=100",
			API_URL,
			slug,
			order.unwrap_or("asc"),
		);

		let result: episodes::Root = self.client.get(&url).send().unwrap().json().unwrap();

		result.data
	}

	/// Gets a specific series information from its slug.
	/// This returns an identical result to those of list_series()
	pub fn get_series<'a>(&self, slug: &'a str) -> series::Series {
		let url = format!("{}/shows/{}", API_URL, slug);

		let mut result: series::Root = self.client.get(&url).send().unwrap().json().unwrap();

		result.data.remove(0)
	}

	pub fn get_episode<'a>(&self, slug: &'a str) -> episodes::Episode {
		let url = format!("{}/watch/{}", API_URL, slug);

		let mut result: episodes::Root = self.client.get(&url).send().unwrap().json().unwrap();

		result.data.remove(0)
	}

	/// Gets an episodes viewing information from its slug.
	/// Please note that this can result in an error if we don't have the permission to
	/// view that video.
	pub fn get_video<'a>(&self, slug: &'a str) -> Result<videos::Video> {
		let url = format!("{}/watch/{}/videos", API_URL, slug);

		let response = self
			.client
			.get(&url)
			// This consumes the header, and HeaderMap doesn't implement Copy
			.headers(self.headers.clone())
			.send()
			.unwrap();

		if response.status().is_success() {
			let mut result: videos::Root = response.json().unwrap();

			Ok(result.data.remove(0))
		} else {
			Err(VideoUnavailable)
		}
	}
}
