use std::fmt::{Display, Formatter, Result};

use crate::album::Album;

pub struct Artist {
    name: String,
    albums: Vec<Album>,
    significant: String,
}

impl Artist {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            albums: Vec::new(),
            significant: String::new(),
        }
    }

    pub fn add_album(&mut self, name: &str) -> () {
        self.albums.push(Album::new(name));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_albums(&self) -> &[Album] {
        &self.albums
    }

    pub fn get_albums_mut(&mut self) -> &mut [Album] {
        &mut self.albums
    }

    pub fn get_album_count(&self) -> usize {
        self.albums.len()
    }

    pub fn get_track_count(&self) -> usize {
        self.albums.iter().map(Album::get_track_count).sum()
    }

    pub fn get_total_length(&self) -> f64 {
        self.albums.iter().map(Album::get_total_length).sum()
    }

    pub fn mark_most_significant(&mut self) -> () {
        self.significant = String::from("*")
    }
}

impl Display for Artist {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "{}{}", self.name, self.significant)?;
        if self.albums.is_empty() {
            return Ok(());
        }
        for album in self.albums.iter() {
            write!(f, "{album}")?;
        }
        Ok(())
    }
}
