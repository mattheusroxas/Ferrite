use youtube_dl::SearchOptions;
use youtube_dl::YoutubeDl;
use youtube_dl::YoutubeDlOutput;

pub fn search() {
    let mut rl = rustyline::Editor::<()>::new();
    let readline = rl.readline(">> ");
    let query = match readline {
        Ok(query) => query,
        Err(_) => {
            println!("No input");
            panic!()
        }
    };
    let page = 20;

    let search = SearchOptions::youtube(&format!("{}&page={}", query, page)).with_count(25);

    let results = YoutubeDl::search_for(&search)
        .run()
        .expect("Failed to run query");

    let videos = match results {
        YoutubeDlOutput::Playlist(playlist) => playlist.entries.expect("No entries ?"),
        YoutubeDlOutput::SingleVideo(video) => vec![*video],
    };

    let titles = videos
        .iter()
        .map(|video| video.title.as_str())
        .collect::<Vec<_>>();

    println!("{}", titles.join("\n"));
}
