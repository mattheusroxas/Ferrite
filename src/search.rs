use num::One;
use rustyline::{Config, EditMode, Editor};
use std::ops::Add;
use youtube_dl::{SearchOptions, YoutubeDl, YoutubeDlOutput};
extern crate num;
use std::io::{stdin, ErrorKind};
use termion::event::Key;
use termion::input::TermRead;

// Input
pub fn search() {
    let config = Config::builder().edit_mode(EditMode::Vi).build();

    let mut rl: Editor<()> = Editor::with_config(config);
    let query = rl.readline("Search for... ").expect("No input");

    // Actual searching
    let mut page = 1;

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

    // Select option
    let _option = rl.readline("Select... ").expect("No input");
    let stdin = stdin();
    for vibecheck in stdin.keys() {
        match vibecheck {
            Ok(Key::Char('\n')) => inc(&mut page),
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => panic!("oh no {:?}", e),
        }
    }
}

pub fn inc<T: Copy + One + Add<T, Output = T>>(x: &mut T) {
    *x = *x + T::one();
}
