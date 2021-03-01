use serde::Deserialize;
use structopt::StructOpt;

// Subcommands
#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(about = "Videos and stuff, duh")]
    Videos(Videos),

    #[structopt(about = "Search for stuff")]
    Search(Search),

    #[structopt(about = "Filter content")]
    Filters(Filters),
}

// Videos
#[derive(Debug, StructOpt)]
pub enum Videos {
    #[structopt(about = "List videos uploaded by a specific channel or user")]
    Uploads,

    #[structopt(about = "List the most popular videos from a specific channel")]
    Popular,

    #[structopt(about = "List the favorite videos of a specific user")]
    Favorites,

    #[structopt(about = "Play YouTube videos by their IDs")]
    ID,

    #[structopt(about = "Show related videos for a video ID or URL")]
    Related,

    #[structopt(about = "List playlists created by a specific channel or user")]
    Playlists,

    #[structopt(about = "Show trending videos in a given category")]
    Trending,

    #[structopt(about = "Display comments for a video by ID or URL")]
    Comments,
}

// Search
#[derive(Debug, StructOpt)]
pub enum Search {
    #[structopt(about = "Search for YouTube videos (default mode)")]
    Search,

    #[structopt(about = "Search for playlists of videos")]
    SearchPlaylists,

    #[structopt(about = "Search for YouTube channels")]
    SearchChannels,
}

// Filters
#[derive(Debug, StructOpt)]
pub enum Filters {
    #[structopt(about = "Videos uploaded by a specific user")]
    Author,

    #[structopt(about = "Videos a specific length")]
    Duration,

    #[structopt(about = "Videos with or without closed captions")]
    Captions,

    #[structopt(
        about = "Use a specific sorting method, valid values: relevance rating upload_date view_count"
    )]
    Order,

    #[structopt(about = "Videos published in a time period")]
    Time,

    #[structopt(about = "Only videos available in at least 720p")]
    Hd,

    #[structopt(about = "Video dimension (any or 3d)")]
    Dimension,

    #[structopt(about = "Video license (any or creative_commons)")]
    Licensce,

    #[structopt(about = "Videos starting from a certain page number")]
    Page,

    #[structopt(about = "high frame rate (HFR) videos")]
    Hfr,

    #[structopt(
        about = "Resolutions: best, 2160p, 1440p, 1080p, 720p, 480p, 360p, 240p, 144p, audio."
    )]
    Resolutions,
}

// Configuration
#[derive(Debug, Clone, Deserialize)]
pub struct General {
    pub api_host: bool,
    pub auto_captions: bool,
    pub autoplay: bool,
}
