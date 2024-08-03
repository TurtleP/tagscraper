use std::fmt::{Display, Formatter, Result};

use crate::track::Track;

pub struct Album {
    title: String,
    tracks: Vec<Track>,
}

impl Album {
    pub fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            tracks: Vec::new(),
        }
    }

    pub fn add_track(&mut self, title: &str, length: f64) {
        self.tracks.push(Track::new(title, length));
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_track_count(&self) -> usize {
        self.tracks.len()
    }

    pub fn get_total_length(&self) -> f64 {
        self.tracks.iter().map(|t| t.get_length()).sum()
    }
}

impl Display for Album {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "  {}", self.title)?;
        if self.tracks.is_empty() {
            return Ok(());
        }
        for track in self.tracks.iter() {
            write!(f, "    {track}")?;
        }
        Ok(())
    }
}

impl PartialEq for Album {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}
