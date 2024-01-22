pub struct AlbumData {
    pub album: String,
    pub tracks: Vec<String>,
    pub year: String,
}
pub struct TagData {
    pub artist: String,
    pub albums: Vec<AlbumData>,
}

impl TagData {
    pub fn new(artist: String, album: String, title: String, year: String) -> TagData {
        let mut albums = Vec::new();

        albums.push(AlbumData {
            album,
            tracks: vec![title],
            year,
        });

        return TagData { artist, albums };
    }

    pub fn add_track(&mut self, album: String, title: String, year: String) {
        for a in &mut self.albums {
            if a.album == album && !a.tracks.contains(&title) {
                a.tracks.push(title);
                return;
            }
        }

        self.albums.push(AlbumData {
            album,
            tracks: vec![title],
            year,
        });
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("{}\n", self.artist));

        for a in &self.albums {
            output.push_str(&format!("  {}\n", a.album));

            for t in &a.tracks {
                output.push_str(&format!("    {}\n", t));
            }
        }

        output.push_str("\n");

        return output;
    }

    pub fn total_tracks(&self) -> usize {
        let mut total = 0;

        for a in &self.albums {
            total += a.tracks.len();
        }

        return total;
    }
}

pub fn from_tag(
    tag: &std::boxed::Box<dyn audiotags::AudioTag>,
) -> (String, String, String, String) {
    let artist = match tag.album_artist() {
        Some(x) => String::from(x),
        None => String::from("Unknown Artist"),
    };

    let album = match tag.album_title() {
        Some(x) => String::from(x),
        None => String::from("Unknown Album"),
    };

    let title = match tag.title() {
        Some(x) => String::from(x),
        None => String::from("Unknown Title"),
    };

    let year = match tag.year() {
        Some(x) => x.to_string(),
        None => String::from("Unknown Year"),
    };

    return (artist, album, title, year);
}
