import sys
from argparse import ArgumentParser
from pathlib import Path

from . import __description__, __prog_name__, __version__, scanner
from .strings import Strings

__CWD_PATH__ = Path().cwd()
__DEFAULT_MUSIC__ = Path("~/Music").expanduser()


def main():
    parser = ArgumentParser(prog=__prog_name__, description=__description__)

    group = parser.add_mutually_exclusive_group()
    group.add_argument("--dir", help=Strings.DIR_FLAG_HELP,
                       default=__DEFAULT_MUSIC__)
    group.add_argument("--list", help=Strings.LIST_FLAG_HELP)

    parser.add_argument("--out", help=Strings.OUT_FLAG_HELP,
                        default=__CWD_PATH__)
    parser.add_argument(
        "--filename", help=Strings.FILENAME_FLAG_HELP, default="music")

    parser.add_argument("--version", help=Strings.VER_FLAG_HELP, action="version",
                        version=f"{__prog_name__} {__version__}")

    parsed = parser.parse_args()

    if hasattr(parsed, "version"):
        return sys.exit(0)

    data = None

    if parsed.dir:
        data = scanner.scan(parsed.dir)
    elif parsed.list:
        data = scanner.collect_file_data(parsed.list.split(","))

    scanner.output_file(data, parsed.out, parsed.filename)


if __name__ == "__main__":
    main()
