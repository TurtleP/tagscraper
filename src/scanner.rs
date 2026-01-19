use std::path::PathBuf;

use crate::collection::Collection;

use anyhow::Result;
use lofty::{
    self,
    file::TaggedFileExt,
    tag::{Accessor, ItemKey},
};
use walkdir::WalkDir;

pub fn scan(directory: &PathBuf) -> Result<Collection> {
    let mut collection = Collection::default();
    let walk_dir = WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());

    for entry in walk_dir {
        let file = match lofty::read_from_path(entry.path()) {
            Ok(file) => file,
            Err(_) => continue,
        };
        let tag = match file.primary_tag() {
            Some(tag) => tag,
            None => continue,
        };
        let artist = tag
            .get_string(&ItemKey::AlbumArtist)
            .unwrap_or("Unknown Artist");
        let album = tag
            .get_string(&ItemKey::AlbumTitle)
            .unwrap_or("Unknown Album");
        let track = tag
            .get_string(&ItemKey::TrackTitle)
            .unwrap_or("Unknown Track");
        let track_number = tag.track().unwrap_or(0);
        collection.add_track(&artist, &album, &track, track_number);
    }
    Ok(collection)
}
