from argparse import ArgumentParser

from musicscraper import __version__, __prog_name__, __description__
from musicscraper.scraper import Scraper


def main():
    parser = ArgumentParser(prog=__prog_name__, description=__description__)

    parser.add_argument(
        "--dir", "-d", help="directory to scan for audio files")
    parser.add_argument(
        "--out", "-o", help="output directory for music.txt")

    parser.add_argument("--version", action='version',
                        version=f"%(prog)s {__version__}")

    parsed = parser.parse_args()

    path = str()
    if parsed.dir:
        path = parsed.dir

    output = str()
    if parsed.out:
        output = parsed.out

    Scraper(path).output(output)


if __name__ == "__main__":
    main()
