import argparse
import sys
from datetime import datetime
from pathlib import Path
from threading import Thread

import taglib
from progress.spinner import MoonSpinner

from musicscraper import __version__

from .artist import Artist

_DEFAULT_PATH = Path.home() / "Music"
_INCLUDE = [
    ".m4a", ".mp3", ".flac",
    ".mpc", ".wav", ".aiff",
    ".ogg"
]

_artists = dict()


def add_metadata(artist, metadata):
    album_info = " ".join(metadata["ALBUM"])
    _artists[artist].add_album(album_info)

    song_info = " ".join(metadata["TITLE"])
    _artists[artist].add_song(album_info, song_info)


def get_artist_totals(artist, totals):
    totals[1] += artist.get_album_count()
    totals[2] += artist.get_songs_count()


def run_scraper(DIRECTORY):
    files = [p for p in Path(DIRECTORY).rglob('*') if p.suffix in _INCLUDE]
    output = f"# musicscraper version {__version__} #\n\n"

    for song in files:
        metadata = taglib.File(str(song)).tags
        artist = " ".join(metadata["ARTIST"])

        # First song in the Artist listing
        if not artist in _artists:
            _artists[artist] = Artist(artist)
            add_metadata(artist, metadata)
        else:
            add_metadata(artist, metadata)

    totals = [len(_artists.keys()), 0, 0]
    for artist in _artists:
        output += _artists[artist].generate_output()
        get_artist_totals(_artists[artist], totals)

    compile_info = str(datetime.now().strftime("%Y-%m-%d %H:%M:%S"))
    output += f"Compiled Date & Time: {compile_info}\n"
    output += f"{totals[0]} Artists • {totals[1]} Albums • {totals[2]} Songs"

    with open("music.txt", "w", encoding="utf-8") as file:
        file.write(output)


def main(args=None):
    PARSER = argparse.ArgumentParser(
        description="Output music info to a file.", prog="musicscraper")

    PARSER.add_argument("--dir", help="The directory to scan")
    PARSER.add_argument("--version", action='version',
                    version=f"%(prog)s {__version__}")

    ARGS = PARSER.parse_args()
    DIRECTORY = Path(ARGS.dir or _DEFAULT_PATH)

    if DIRECTORY.exists():
        start = datetime.now()

        thread = Thread(target=run_scraper, args=(DIRECTORY,), daemon=True)
        spinner = MoonSpinner('Parsing audio files.. ')
        thread.start()

        while thread.is_alive():
            spinner.next()

        thread.join()

        end = (datetime.now() - start)
        print(f"\nFinished in {end.seconds}s")
    else:
        print(f"Failed to parse directory: '{DIRECTORY}'. Does not exist.")
        return


if __name__ == "__main__":
    main(sys.argv)
