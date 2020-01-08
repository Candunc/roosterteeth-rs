use crate::requests::Requests;

#[test]
fn list_channels() {
	let requests = Requests::new();
	let channels = requests.list_channels();
	assert_eq!(
		&channels[0].attributes.name,
		"Rooster Teeth");

	assert_eq!(
		&channels[1].attributes.slug,
		"achievement-hunter");
}

#[test]
fn list_episodes() {
	let requests = Requests::new();
	let episodes = requests.list_episodes(1, None, Some("asc"));

	assert_eq!(
		&episodes[0].uuid,
		"ffabf033-464d-11e7-a302-065410f210c4"
	);

	assert_eq!(
		&episodes[1].attributes.slug,
		"red-vs-blue-season-2-episode-22"
	);
}

#[test]
fn list_series() {
	let requests = Requests::new();
	let series = requests.list_series(None, Some("asc"));

	assert_eq!(
		&series[0].uuid,
		"ff9265c3-464d-11e7-a302-065410f210c4"
	);

	assert_eq!(
		&series[1].attributes.slug,
		"1-800-magic"
	);
}



#[test]
fn get_seasons() {
	let requests = Requests::new();
	let seasons = requests.get_seasons("red-vs-blue", Some("asc"));

	assert_eq!(
		seasons[0].attributes.number,
		1
	);

	assert_eq!(
		&seasons[1].attributes.slug,
		"red-vs-blue-season-2"
	);
}

#[test]
fn get_season_episodes() {
	let requests = Requests::new();
	let episodes = requests.get_season_episodes("red-vs-blue-season-1");

	assert_eq!(
		&episodes[0].attributes.title,
		"Episode 1: Why Are We Here?"
	);

	assert_eq!(
		&episodes[1].attributes.slug,
		"red-vs-blue-season-1-episode-2"
	);
}

#[test]
fn get_series() {
	let requests = Requests::new();
	let series = requests.get_series("red-vs-blue");

	assert_eq!(
		&series.uuid,
		"ff925ff9-464d-11e7-a302-065410f210c4"
	);

	assert_eq!(
		&series.attributes.category,
		"episodic"
	);
}

#[test]
fn get_video() {
	let requests = Requests::new();
	let video = requests.get_video("million-dollars-but-season-1-magic-dogs-and-muscle-men").unwrap();

	assert_eq!(
		&video.uuid,
		"48bb3e93-04ea-4f90-a6ec-f9aff2c79dfa"
	);

	assert_eq!(
		&video.attributes.content_uuid,
		"0006f0d4-464e-11e7-a302-065410f210c4"
	);
}
