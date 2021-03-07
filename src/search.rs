use rustyline::{Config, EditMode, Editor};
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use youtube_dl::{SearchOptions, YoutubeDl, YoutubeDlOutput};

// Input
pub fn search() {
    let config = Config::builder().edit_mode(EditMode::Vi).build();
    let mut rl: Editor<()> = Editor::with_config(config);

    let query = rl.readline("Search for... ").expect("No input");

    let mut page = 1;

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

    // Select option
    let _option = rl.readline("Select... ").expect("No input");
    let mut stdout = stdout().into_raw_mode().unwrap();
    for vibecheck in stdin().keys() {
        // Clear the *entire* screen before printing the below results
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        match vibecheck.unwrap() {
            Key::Char('\n') => {
                page += 1;

                let search =
                    SearchOptions::youtube(&format!("{}&page={}", query, page)).with_count(5);

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

                write!(stdout, "{}", titles.join("\n")).unwrap(); // TODO use more efficent method, https://stackoverflow.com/a/56037073
            }

            _ => break,
        }
    }
}
