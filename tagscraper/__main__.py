from argparse import ArgumentParser

from musicscraper import __description__, __prog_name__, __version__
from musicscraper.scraper import Scraper


def main():
    parser = ArgumentParser(prog=__prog_name__, description=__description__)

    parser.add_argument(
        "--dir", "-d", help="directory to scan for audio files", default=Scraper._DEFAULT_PATH)

    parser.add_argument(
        "--out", "-o", help="output directory for music.txt", default=Scraper._DEFAULT_OUTPUT_PATH)

    parser.add_argument("--version", action='version',
                        version=f"%(prog)s {__version__}")

    parsed = parser.parse_args()

    _ = Scraper(parsed.dir, parsed.out)


if __name__ == "__main__":
    main()
