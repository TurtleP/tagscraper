use audiotags::Tag;
use clap::Parser;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

mod tags;
use self::tags::Info;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Commands {
    #[arg(short, long, default_value = ".")]
    directory: Option<PathBuf>,

    #[arg(short, long, default_value = "music.txt")]
    filepath: Option<PathBuf>,
}

fn main() -> Result<(), i32> {
    let args = Commands::parse();

    /* we can expect the directory to be default */
    let root_path = args.directory.unwrap();
    let mut artist_info: HashMap<String, Info> = HashMap::new();

    /* create a way to iterate through the directory recursively */
    let walk_iter = WalkDir::new(root_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|item| item.file_type().is_file() && item.path().extension().is_some());

    /* walk the directory */
    for entry in walk_iter {
        /* get the audio tag from the file - only if it's valid */
        let tag = match Tag::new().read_from_path(entry.path()) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let album_artist = tag.album_artist().ok_or(-1)?;
        let data = artist_info
            .entry(String::from(album_artist))
            .or_insert(Info::new());

        let album_name = tag.album_title().ok_or(-2)?;
        // let album_year = tag.year().ok_or(-3)?;

        let song_title = tag.title().ok_or(-3)?;

        data.add_song(album_name, song_title)
    }

    let filepath = args.filepath.ok_or(-4)?;
    let _ = write_output(&filepath, &artist_info);

    return Ok(());
}

fn write_output(filename: &PathBuf, info: &HashMap<String, Info>) -> io::Result<()> {
    let mut file = File::create(filename)?;

    for artist in info.keys().sorted_by_key(|key| key.to_lowercase()) {
        writeln!(file, "{}", artist)?;
        for album in info[artist].get_album_info().keys() {
            writeln!(file, "  {}", album)?;
            for index in 0..info[artist].get_song_count(album) {
                writeln!(file, "    {}", info[artist].get_song(album, index))?;
            }
        }
    }

    let artist_count = info.keys().len();
    let albums_count: usize = info.values().map(|x| x.get_album_count()).sum();

    let songs_count: usize = info.values().map(|data| data.get_total_songs()).sum();

    writeln!(
        file,
        "\n{} Artist(s) / {} Album(s) / {} Song(s)",
        artist_count, albums_count, songs_count
    )?;

    Ok(())
}
