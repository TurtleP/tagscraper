use clap::Parser;

use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use audiotags::Tag;
use shellexpand;
use walkdir::WalkDir;

mod tags;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const APPNAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser)]
#[clap(version = VERSION)]
struct Opts {
    #[clap(default_value = "~/Music")]
    dir: String,

    #[clap(short, long, default_value = ".")]
    out: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    /* expand the tilde, if necessary */
    let expanded = shellexpand::tilde(&opts.dir).into_owned();

    /* clone the string */
    let value = expanded.clone();
    let path = Path::new(OsStr::new(&value));

    if !path.is_dir() {
        return println!("Path '{}' does not exist. Aborting.", opts.dir);
    }

    let mut seen: Vec<String> = Vec::new();
    let file = match File::create(format!("{}/music.txt", opts.out)) {
        Ok(x) => x,
        Err(_) => return,
    };

    let mut stats = tags::Stats {
        artists: 0,
        albums: 0,
        songs: 0,
    };

    for entry in WalkDir::new(path.as_os_str()) {
        let dir_entry = match entry {
            Ok(x) => x,
            Err(_) => {
                println!("Failed to read entry!");
                break;
            }
        };

        if dir_entry.path().is_file() && dir_entry.path().extension().is_some() {
            let audio_tag = Tag::default();

            let meta_data = match audio_tag.read_from_path(dir_entry.path()) {
                Ok(x) => x,
                Err(_) => continue,
            };

            let (artist, album, title, _year) = tags::from_tag(meta_data);

            let arist_val = artist.clone();
            if !seen.contains(&arist_val) {
                if seen.len() > 0 {
                    write(&file, String::new(), 0);
                }

                write(&file, artist, tags::Indentation::IndentArtist.into());

                stats.artists += 1;
                seen.push(arist_val);
            }

            let album_val = album.clone();
            if !seen.contains(&album_val) {
                write(&file, album, tags::Indentation::IndentAlbum.into());

                stats.albums += 1;
                seen.push(album_val);
            }

            stats.songs += 1;
            write(&file, title, tags::Indentation::IndentTitle.into());
        }
    }

    /* fancy output thing */

    let values = format!(
        "Artists {} • Albums {} • Songs {}",
        stats.artists, stats.albums, stats.songs
    );

    let app_meta: String = format!("{} {}", APPNAME, VERSION);
    let metadata = format!("# {:-^width$} #", app_meta, width = values.chars().count());

    let footer = format!("# {:-^width$} #", "", width = values.chars().count());

    write(&file, String::new(), 0);

    write(&file, metadata, 0);
    write(&file, format!("# {} #", values), 0);
    write(&file, footer, 0)
}

fn write(mut file: &File, data: String, indentation: usize) {
    let indent = String::from_utf8(vec![b' '; indentation]).unwrap();
    let result = format!("{}{}\n", indent, data);

    file.write_all(result.as_bytes())
        .expect("Cannot write to file.");
}
