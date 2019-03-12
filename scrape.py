import json
import os
from datetime import datetime
from pathlib import Path

import taglib

__music_dict = dict()
__include = [
    ".m4a", ".mp3", ".flac",
    ".mpc", ".wav", ".aiff",
    ".ogg"
]
__music_directory = str(Path.home()) + "/Music"
__counts = [0, 0, 0]


def __scrape(music_data, file):
    metadata = taglib.File(file).tags

    artist_info = " ".join(metadata["ARTIST"])
    if artist_info not in music_data:
        music_data[artist_info] = dict()
        __counts[0] += 1

    album_info = " ".join(metadata["ALBUM"])
    if album_info not in music_data[artist_info]:
        music_data[artist_info][album_info] = list()
        __counts[1] += 1

    song_info = " ".join(metadata["TITLE"])
    if song_info not in music_data[artist_info][album_info]:
        music_data[artist_info][album_info].append(song_info)
        __counts[2] += 1

    return


def find(directory=None):
    if directory is None:
        directory = __music_directory

    files = os.listdir(directory)

    path = directory + "/"

    for index in range(len(files)):
        if os.path.isdir(path + files[index]):
            find(path + files[index])
        else:
            if files[index][-4:] in __include:  # Avoid things like .DS_Store
                __scrape(__music_dict, path + files[index])

    return __music_dict


def dump(music_data, filename="music.txt"):
    with open(filename, "w") as file:
        json.dump(music_data, file, indent=2)

        file.write("\n" * 2)

        compile_info = str(datetime.now().strftime("%Y-%m-%d %H:%M:%S"))
        file.write("\nCompiled Date & Time: " + compile_info + "\n")

        artist_count = str(__counts[0]) + " Artists • "
        album_count = str(__counts[1]) + " Albums • "
        song_count = str(__counts[2]) + " Songs"

        file.write(artist_count + album_count + song_count)

    return
