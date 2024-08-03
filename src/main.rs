use audiotags::{AudioTag, Tag};
use std::cmp;
use std::io::Write;
use std::path::Path;
use std::{env, time::Duration};

use walkdir::WalkDir;

mod album;
mod artist;
mod track;

use artist::Artist;

fn get_album(tag: &(dyn AudioTag + Send + Sync)) -> &str {
    if let Some(album) = tag.album() {
        return album.title;
    }
    "Unknown"
}

fn get_track_title(tag: &(dyn AudioTag + Send + Sync)) -> &str {
    if let Some(title) = tag.title() {
        return title;
    }
    "Unknown"
}

pub fn get_track_length(tag: &(dyn AudioTag + Send + Sync)) -> f64 {
    if let Some(length) = tag.duration() {
        return length;
    }
    0.0f64
}

fn get_total_time_days_hrs_mins_secs(time: f64) -> (u64, u64, u64, u64) {
    let total_time = Duration::from_secs_f64(time).as_secs();

    let days = total_time / (60 * 60 * 24);
    let hours = (total_time % (60 * 60 * 24)) / (60 * 60);
    let minutes = (total_time % (60 * 60)) / 60;
    let seconds = total_time % 60;

    (days, hours, minutes, seconds)
}

fn main() -> Result<(), String> {
    let mut data: Vec<Artist> = Vec::new();
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Usage: audiotag <directory>".to_string());
    }

    let directory = Path::new(&args[1]);
    if !Path::exists(directory) {
        return Err("Directory does not exist".to_string());
    }

    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| !entry.file_type().is_dir())
    {
        let tag = match Tag::new().read_from_path(entry.path()) {
            Ok(tag) => tag,
            Err(_) => continue,
        };

        if let Some(artist) = tag.artist() {
            let album_name = get_album(&*tag);

            // If the artist is already in the data, add the album to the artist's list of albums
            if let Some(instance) = data.iter_mut().find(|a| a.get_name() == artist) {
                if !instance
                    .get_albums()
                    .iter()
                    .any(|a| a.get_title() == album_name)
                {
                    instance.add_album(album_name);
                }
            } else {
                // Otherwise, create a new artist and add the album to the artist's list of albums
                let mut new_artist = Artist::new(artist);
                new_artist.add_album(album_name);
                data.push(new_artist);
            }

            // Add the track to the album
            if let Some(instance) = data.iter_mut().find(|a| a.get_name() == artist) {
                if let Some(album) = instance
                    .get_albums_mut()
                    .iter_mut()
                    .find(|a| a.get_title() == album_name)
                {
                    album.add_track(get_track_title(&*tag), get_track_length(&*tag));
                }
            }
        }
    }

    if data.is_empty() {
        return Err("No audio files found".to_string());
    }

    let mut count: usize = 0;
    let mut str = String::new();

    data.iter().for_each(|x| {
        let prev = count;
        count = cmp::max(count, x.get_track_count());
        if count > prev {
            str = x.get_name().to_string();
        }
    });

    if let Some(v) = data.iter_mut().find(|x| x.get_name() == str) {
        v.mark_most_significant();
    }

    dump_file("output.txt", &data)
}

fn dump_file(path: &str, data: &[Artist]) -> Result<(), String> {
    let mut file: std::fs::File = match std::fs::File::create(path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };

    for artist in data {
        let _ = file.write_all(format!("{}\n", artist).as_bytes());
    }

    let artists = data.len();

    let albums: usize = data.iter().map(|a| a.get_album_count()).sum();
    let tracks_count: usize = data.iter().map(|a| a.get_track_count()).sum();

    let stats = format!("{artists} Artists - {albums} Albums - {tracks_count} Tracks\n");
    file.write_all(stats.as_bytes()).ok();

    let total_duration: f64 = data.iter().map(|a| a.get_total_length()).sum();
    let (days, hours, mins, secs) = get_total_time_days_hrs_mins_secs(total_duration);

    let formatted_time = format!(
        "Duration: {} days, {} hours, {} minutes, {} seconds\n",
        days, hours, mins, secs
    );
    file.write_all(formatted_time.as_bytes()).ok();

    Ok(())
}
