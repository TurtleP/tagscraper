from enum import Enum


class Strings(str, Enum):
    DIR_FLAG_HELP = "directory to scan for audio files"
    OUT_FLAG_HELP = "output directory for music.txt"
    LIST_FLAG_HELP = "input of items, separated by commas"
    VER_FLAG_HELP = "show version and exit"
    FILENAME_FLAG_HELP = "name of the output file"
