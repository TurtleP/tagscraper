class Artist:

    ALBUM_NAME_SPACES = " " * 2
    SONG_TITLE_SPACES = " " * 4

    def __init__(self, name):
        self.name = name
        self.albums = dict()

    def add_album(self, info):
        if not info in self.albums:
            self.albums[info] = list()

    def add_song(self, album, info):
        if not info in self.albums[album]:
            self.albums[album].append(info)

    def get_album_count(self):
        return len(self.albums.keys())

    def get_songs_count(self):
        total = 0

        for album in self.albums:
            total += len(self.albums[album])

        return total

    def generate_output(self):
        output = f"{self.name}\n"

        for album in self.albums:
            output += f"{Artist.ALBUM_NAME_SPACES}{album}\n"
            for song in self.albums[album]:
                output += f"{Artist.SONG_TITLE_SPACES}{song}\n"

        return output + "\n"