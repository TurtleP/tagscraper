import os
from datetime import datetime
from pathlib import Path

import taglib

from . import __version__, __prog_name__


class Scraper:

    _DEFAULT_PATH = Path().home() / "Music"
    _DEFAULT_OUTPUT_PATH = Path().cwd()

    _INCLUDE = [
        ".m4a", ".mp3", ".flac",
        ".mpc", ".wav", ".aiff",
        ".ogg"
    ]

    __HEADER = "#{0:-^d}"
    __STATS = "# {0}"
    __FOOTER = "#{0:-^d}"

    def __init__(self, directory: str) -> None:
        """Initialize the scraper with an optional @directory."""

        path = Path(directory)
        if directory and not path.exists():
            return print(f"Directory {path} does not exist.")
        else:
            path = Scraper._DEFAULT_PATH

        self.music = dict()
        self.__scan_directory(path)

    def __add_data(self, filepath: str) -> None:
        """Adds metdata from @filepath to the music dict."""

        meta_tag = taglib.File(filepath).tags

        # Note: use ALBUMARTIST because ARTIST includes extra data #
        # Most, if not all music players group their Artist list via that #
        artist_name = " ".join(meta_tag["ALBUMARTIST"])
        album_name = " ".join(meta_tag["ALBUM"])
        song_name = " ".join(meta_tag["TITLE"])

        # Make sure that the Artist isn't already added #
        if not artist_name in self.music:
            self.music[artist_name] = dict()

        # Also ensure the Album doesn't already exist #
        if not album_name in self.music[artist_name]:
            self.music[artist_name][album_name] = list()

        # Add the song title to the listing #
        self.music[artist_name][album_name].append(song_name)

    def __scan_directory(self, directory: Path) -> None:
        """Scan the @directory specified for taggable items"""

        for filepath in directory.rglob("*"):
            if not filepath.suffix in Scraper._INCLUDE:
                continue

            self.__add_data(str(filepath))

    def __get_stats(self) -> str:
        """Get the overall count of artists, albums, and songs as a str"""

        stats = "Artists {} • Albums {} • Songs {}"

        artist_count = len(self.music.keys())
        album_count = sum(len(artist) for artist in self.music.values())
        song_count = sum(len(song_list) for _,
                         artist_dict in self.music.items() for _, song_list in artist_dict.items())

        return stats.format(artist_count, album_count, song_count)

    def __build_footer(self) -> list:
        # Build stats first
        __totals = self.__get_stats()
        __stats = Scraper.__STATS.format(__totals)

        __time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        Scraper.__HEADER = Scraper.__HEADER.replace("d", str(len(__stats)))
        __header = Scraper.__HEADER.format(__time)

        __versioning = __prog_name__ + " " + __version__
        Scraper.__FOOTER = Scraper.__FOOTER.replace("d", str(len(__stats)))
        __footer = Scraper.__FOOTER.format(__versioning)

        return [__header, __stats, __footer]

    def output(self, directory: str) -> None:
        """Write the final output into @directory/music.txt"""

        if not directory or os.path.exists(directory):
            directory = Scraper._DEFAULT_OUTPUT_PATH

        with open(f"{directory}/music.txt", "w", encoding="utf-8") as file:
            for artist, album_dict in self.music.items():
                print(artist, file=file)
                for album, song_list in album_dict.items():
                    print(f"  {album}", file=file)
                    for song_title in song_list:
                        print(f"    {song_title}", file=file)

                print(file=file)

            print("\n".join(self.__build_footer()), file=file)
