/*!
RoosterTeeth-rs is a rust wrapper for the RoosterTeeth VOD api. All requests are done through the [requests](./requests/struct.Requests.html) object.

There are usually optional parameters to restrict to a certain channel or to change how it is sorted.
In the following example, we grab the first page of episodes, with no restrictions and default sorting.

```no_run
let requests = roosterteeth_rs::requests::Requests::new();

let episodes = requests.list_episodes(1, None, None);

println!("{}",episodes[0].attributes.title);

```

All of the returned values are documented in the following page as structs:
* [Channels](./structs/channels/struct.Channel.html)
* [Episodes](./structs/episodes/struct.Episode.html)
* [Seasons](./structs/seasons/struct.Season.html)
* [Series](./structs/series/struct.Series.html)
* [Videos](./structs/videos/struct.Video.html)

Please note the difference between an [Episode](./structs/episode/struct.Episode.html) and a [Video](./structs/video/struct.Video.html) struct.
An episode struct returns all the information about that episode, while a video struct is useful mainly for getting the m3u8 urls and 
will fail if you don't have permissions to watch the video. (For example, if you aren't a first member or the video isn't public.)
*/

pub mod requests;
pub mod structs;

#[cfg(test)]
mod tests;
