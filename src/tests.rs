use crate::requests::{Credential, Requests};

#[test]
fn list_channels() {
	let requests = Requests::new(Credential::Anonymous);
	let channels = requests.list_channels();
	assert_eq!(&channels[0].attributes.name, "Rooster Teeth");

	assert_eq!(&channels[1].attributes.slug, "achievement-hunter");
}

#[test]
fn list_episodes() {
	let requests = Requests::new(Credential::Anonymous);
	let episodes = requests.list_episodes(1, None, Some("asc"));

	assert_eq!(&episodes[0].uuid, "ffabf033-464d-11e7-a302-065410f210c4");

	assert_eq!(
		&episodes[1].attributes.slug,
		"red-vs-blue-season-2-episode-22"
	);
}

#[test]
fn list_series() {
	let requests = Requests::new(Credential::Anonymous);
	let series = requests.list_series(None, Some("asc"));

	assert_eq!(&series[0].uuid, "ff9265c3-464d-11e7-a302-065410f210c4");

	assert_eq!(&series[1].attributes.slug, "1-800-magic");
}

#[test]
fn get_seasons() {
	let requests = Requests::new(Credential::Anonymous);
	let seasons = requests.get_seasons("red-vs-blue", Some("asc"));

	assert_eq!(seasons[0].attributes.number, 1);

	assert_eq!(&seasons[1].attributes.slug, "red-vs-blue-season-2");
}

#[test]
fn get_season_episodes() {
	let requests = Requests::new(Credential::Anonymous);
	let episodes = requests.get_season_episodes("red-vs-blue-season-1", None);

	assert_eq!(&episodes[0].attributes.title, "Episode 1: Why Are We Here?");

	assert_eq!(
		&episodes[1].attributes.slug,
		"red-vs-blue-season-1-episode-2"
	);
}

#[test]
fn get_series() {
	let requests = Requests::new(Credential::Anonymous);
	let series = requests.get_series("red-vs-blue");

	assert_eq!(&series.uuid, "ff925ff9-464d-11e7-a302-065410f210c4");

	assert_eq!(&series.attributes.category, "episodic");
}

#[test]
fn get_episode() {
	let requests = Requests::new(Credential::Anonymous);
	let episode = requests.get_episode("million-dollars-but-season-1-magic-dogs-and-muscle-men");

	assert_eq!(&episode.uuid, "0006f0d4-464e-11e7-a302-065410f210c4");

	assert_eq!(&episode.attributes.title, "Magic Dogs & Muscle Men");
}

#[test]
fn get_video() {
	let requests = Requests::new(Credential::Anonymous);
	let video = requests
		.get_video("million-dollars-but-season-1-magic-dogs-and-muscle-men")
		.unwrap();

	assert_eq!(&video.uuid, "48bb3e93-04ea-4f90-a6ec-f9aff2c79dfa");

	assert_eq!(
		&video.attributes.content_uuid,
		"0006f0d4-464e-11e7-a302-065410f210c4"
	);
}

#[test]
fn process_new_videos() {
	/*
	We can't actually process anything because we are comparing against live
	data - nothing predetermined. However, by processing the latest episode of
	something, any newly implemented API that were previously incorrectly typed
	will now fail upon running this test.
	*/
	let requests = Requests::new(Credential::Anonymous);

	let seasons = requests.get_seasons("rt-animated-adventures", None);
	let episodes = requests.get_season_episodes(&seasons[0].attributes.slug, None);

	assert_eq!(&episodes[0].attributes.show_slug, "rt-animated-adventures");
}
