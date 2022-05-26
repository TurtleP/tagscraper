from pathlib import Path

import taglib

from . import __prog_name__, __version__

__SAFE_FILES__ = [".m4a", ".mp3", ".flac",
                  ".mpc", ".wav", ".aiff",
                  ".ogg"]


def get_file_data(filepath=str) -> tuple:
    data = taglib.File(filepath.as_posix()).tags

    return (data["ALBUMARTIST"][0], data["ALBUM"][0], data["TITLE"][0])


def collect_file_data(items=list) -> dict:
    root = dict()

    for filepath in items:
        tag_info = get_file_data(filepath)

        if not tag_info[0] in root:
            root[tag_info[0]] = dict()

        if not tag_info[1] in root[tag_info[0]]:
            root[tag_info[0]][tag_info[1]] = list()

        root[tag_info[0]][tag_info[1]].append(tag_info[2])

    return root


def scan(directory=Path) -> list:
    items = list()

    for item in directory.rglob("*"):
        if not item.suffix in __SAFE_FILES__:
            continue

        items.append(item)

    return collect_file_data(items)


def output_file(data=dict, directory=Path, filename=str):
    filepath = directory / f"{filename}.txt"

    artist_count = len(data.keys())
    albums_count = sum(len(artist) for artist in data.values())
    songs_count = sum(len(song_list) for _,
                      artist_dict in data.items() for _, song_list in artist_dict.items())

    meta_info = "# Artists: {} • Albums: {} • Songs: {}"

    with filepath.open("w") as file:
        for artist, album_dict in data.items():
            print(artist, file=file)
            for album, song_list in album_dict.items():
                print(f"  {album}", file=file)
                for song in song_list:
                    print(f"    {song}", file=file)

            print("", file=file)

        print(meta_info.format(artist_count, albums_count, songs_count), file=file)
        print(f"# {__prog_name__} {__version__}", file=file)
