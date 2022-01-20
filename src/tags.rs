use num_enum::IntoPrimitive;

pub struct Stats {
    pub artists: i32,
    pub albums: i32,
    pub songs: i32,
}

#[derive(IntoPrimitive)]
#[repr(usize)]
pub enum Indentation {
    IndentArtist = 0x00,
    IndentAlbum = 0x02,
    IndentTitle = 0x04,
}

pub fn from_tag(tag: std::boxed::Box<dyn audiotags::AudioTag>) -> (String, String, String, String) {
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
