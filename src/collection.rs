use std::collections::BTreeMap;
use std::fmt;

struct Track {
    title: String,
    track_number: u32,
}

type Artist = BTreeMap<String, Vec<Track>>;

#[derive(Default)]
pub struct Collection {
    artists: BTreeMap<String, Artist>,
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (artist, albums) in &self.artists {
            writeln!(f, "{}", artist)?;
            for (album, tracks) in albums {
                writeln!(f, "  {}", album)?;
                for track in tracks {
                    writeln!(f, "    {}", track.title)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Collection {
    pub fn add_track(&mut self, artist: &str, album: &str, track: &str, track_number: u32) {
        let artist = self.artists.entry(artist.to_string()).or_default();
        let album = artist.entry(album.to_string()).or_default();
        album.push(Track {
            title: track.to_string(),
            track_number,
        });
        album.sort_by_key(|track| track.track_number);
    }
}
