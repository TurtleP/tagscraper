use std::collections::HashMap;

pub struct Info {
    album_info: HashMap<String, Vec<String>>,
}

impl Info {
    pub fn new() -> Info {
        return Info {
            album_info: HashMap::new(),
        };
    }

    pub fn add_song(&mut self, album: &str, title: &str) {
        let entry = self
            .album_info
            .entry(String::from(album))
            .or_insert(Vec::new());

        entry.push(String::from(title));
    }

    pub fn get_album_info(&self) -> &HashMap<String, Vec<String>> {
        return &self.album_info;
    }

    pub fn get_song(&self, album: &String, index: usize) -> &String {
        return &self.album_info[album][index];
    }

    pub fn get_album_count(&self) -> usize {
        return self.album_info.len();
    }

    pub fn get_total_songs(&self) -> usize {
        let mut total = 0;
        for (_, song_list) in &self.album_info {
            total += song_list.len();
        }

        return total;
    }

    pub fn get_song_count(&self, album: &String) -> usize {
        return self.album_info[album].len();
    }
}
