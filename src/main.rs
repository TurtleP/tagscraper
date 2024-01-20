use crate::tags::TagData;

use audiotags::Tag;
use clap::Parser;

use std::fs;
use std::io::Write;
use std::path::PathBuf;

mod tags;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const APPNAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser)]
#[clap(version = VERSION, name = APPNAME)]
struct Opts {
    #[clap(help = "Directory to scan")]
    dir: PathBuf,

    #[clap(short, long, default_value = "./music.txt", help = "Output file")]
    out: PathBuf,
}

fn validate_path(path: &PathBuf) {
    if path.is_dir() && path.exists() {
        return;
    }
    println!("Path '{}' does not exist. Aborting.", path.display());
}

fn read_dir(path: &PathBuf, items: &mut Vec<TagData>) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            read_dir(&path, items);
        } else {
            let tag = match Tag::new().read_from_path(&path) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let (artist, album, title, year) = tags::from_tag(&tag);

            if items.is_empty() || items.last().unwrap().artist != tag.album_artist().unwrap() {
                items.push(TagData::new(artist, album, title, year));
            } else {
                items.last_mut().unwrap().add_track(album, title, year);
            }
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    validate_path(&opts.dir);

    let mut results = Vec::new();
    read_dir(&opts.dir, &mut results);

    let mut file = fs::File::create(opts.out).unwrap();

    for r in &results {
        file.write_all(r.to_string().as_bytes()).unwrap();
    }

    let total_tracks: usize = results.iter().map(|x| x.total_tracks()).sum();
    let total_albums: usize = results.iter().map(|x| x.albums.len()).sum();
    let total_artists: usize = results.len();

    let totals = format!(
        "Artists {} • Albums {} • Tracks {}\n",
        total_artists, total_albums, total_tracks
    );

    file.write_all(totals.as_bytes()).unwrap();

    let version_info = format!("tagscraper {}\n", VERSION);
    file.write_all(version_info.as_bytes()).unwrap();
}
