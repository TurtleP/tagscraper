import argparse

import scrape

PARSER = argparse.ArgumentParser(description="Output music info to a file.")
PARSER.add_argument("--dir", help="The directory to scan")

ARGS = PARSER.parse_args()
DIRECTORY = ARGS.dir or None

music_dict = scrape.find(DIRECTORY)
scrape.dump(music_dict)
