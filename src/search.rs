use rustyline::{Config, EditMode, Editor};
use std::io::{stdin, ErrorKind};
use termion::event::Key;
use termion::input::TermRead;
use youtube_dl::{SearchOptions, YoutubeDl, YoutubeDlOutput};

// Input
fn search(page: u32) {
    let config = Config::builder().edit_mode(EditMode::Vi).build();
    let mut rl: Editor<()> = Editor::with_config(config);

    let query = rl.readline("Search for... ").expect("No input");

    // Actual searching
    let search = SearchOptions::youtube(&format!("{}&page={}", query, page)).with_count(5);

    let results = YoutubeDl::search_for(&search)
        .run()
        .expect("Failed to run query");

    let videos = match results {
        YoutubeDlOutput::Playlist(playlist) => playlist.entries.expect("No entries?"),
        YoutubeDlOutput::SingleVideo(video) => vec![*video],
    };

    let titles = videos
        .iter()
        .map(|video| video.title.as_str())
        .collect::<Vec<_>>();

    println!("{}", titles.join("\n")); // TODO use more efficent method, https://stackoverflow.com/a/56037073
}

// Select option
pub fn pager() {
    let config = Config::builder().edit_mode(EditMode::Vi).build();
    let mut rl: Editor<()> = Editor::with_config(config);

    let _option = rl.readline("Select... ").expect("No input");

    let mut page = 1;

    for vibecheck in stdin().keys() {
        match vibecheck {
            Ok(Key::Char('\n')) => {
                page += 1;
                search(page);
            }

            Ok(_) => break,
            Err(e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => panic!("oh no {:?}", e),
        }
    }
}
