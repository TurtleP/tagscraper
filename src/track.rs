use std::fmt::{Display, Formatter, Result};

pub struct Track {
    title: String,
    length: f64,
}

impl Track {
    pub fn new(title: &str, length: f64) -> Self {
        Self {
            title: String::from(title),
            length,
        }
    }

    pub fn get_length(&self) -> f64 {
        self.length
    }
}

impl Display for Track {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "{}", self.title)
    }
}
