import json
import os
from datetime import datetime
from pathlib import Path

import taglib

_music_dict = dict()
_include = [
    ".m4a", ".mp3", ".flac",
    ".mpc", ".wav", ".aiff",
    ".ogg"
]
_music_directory = Path.home() / "Music"
_counts = [0, 0, 0]


def _scrape(music_data, file):
    metadata = taglib.File(file).tags

    artist_info = " ".join(metadata["ARTIST"])
    if artist_info not in music_data:
        music_data[artist_info] = dict()
        _counts[0] += 1

    album_info = " ".join(metadata["ALBUM"])
    if album_info not in music_data[artist_info]:
        music_data[artist_info][album_info] = list()
        _counts[1] += 1

    song_info = " ".join(metadata["TITLE"])
    if song_info not in music_data[artist_info][album_info]:
        music_data[artist_info][album_info].append(song_info)
        _counts[2] += 1


def find(directory=None):
    if directory is None:
        directory = _music_directory

    files = os.listdir(directory)

    for item in files:
        if os.path.isdir(directory / item):
            find(directory / item)
        else:
            if item[-4:] in _include:  # Avoid things like .DS_Store
                _scrape(_music_dict, directory / item)

    return _music_dict


def dump(music_data, filename="music.txt"):
    with open(filename, "w") as file:
        json.dump(music_data, file, indent=2)

        file.write("\n" * 2)

        compile_info = str(datetime.now().strftime("%Y-%m-%d %H:%M:%S"))
        file.write("\nCompiled Date & Time: " + compile_info + "\n")

        artist_count = str(_counts[0]) + " Artists • "
        album_count = str(_counts[1]) + " Albums • "
        song_count = str(_counts[2]) + " Songs"

        file.write(artist_count + album_count + song_count)
