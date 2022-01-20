# tagscraper

A rust application to get ID3 tags and output them to a file.
Based on the original script made by my friend, [MMaker](https://github.com/stysmmaker)

## Dependencies

See [the Cargo.toml file](Cargo.toml). These will be installed automatically when building via `cargo build`.

## Installation

There are a few ways this can be installed. The easiest is to download the binary from the [releases page](https://github.com/TurtleP/tagscraper) and save it to a memorable directory. There are a few alternate methods that also exist.

### Releases Page

Specifically, you will want to save the binary in these places. Realistically it does not matter, but these are the simplest to remember.<br>
Windows users, please add the `tagscraper` path to your PATH environment variable.

- Windows:
  - `%appdata%\tagscraper`
- macOS & Linux:
  - `/usr/bin`


### Git Clone

1. Clone this repository
2. Open your termainal to the new directory
3. Run `cargo install --path .`

### Cargo via Git URL

1. Open your terminal
2. Run `cargo install --git git://github.com/TurtleP/tagscraper`

## Usage

```
usage: musicscraper [-h] [--dir DIR] [--out OUT] [--version]

Music (metadata) Scraper that outputs to a file

optional arguments:
  -h, --help         show this help message and exit
  --dir DIR, -d DIR  directory to scan for audio files
  --out OUT, -o OUT  output directory for music.txt
  --version          show program's version number and exit
```

## Example Output

This was created with m4a files from iTunes. According to pytaglib, it should support other formats, including: FLAC, MPC, Speex, WavPack, TrueAudio, WAV, AIFF, mp4 and ASF files.

```
Bastille
  Bad Blood (Bonus Track Version)
    Pompeii
    Things We Lost in the Fire
    Bad Blood
    Overjoyed
    These Streets
    Weight of Living, Pt. 2
    Icarus
    Oblivion
    Flaws
    Daniel in the Den
    Laura Palmer
    Get Home
    The Silence (Bonus Track)
    Weight of Living, Pt. 1 (Bonus Track)
    Laughter Lines (Bonus Track)
  Doom Days
    Quarter Past Midnight
    Bad Decisions
    The Waves
    Divide
    Million Pieces
    Doom Days
    Nocturnal Creatures
    4AM
    Another Place
    Those Nights
    Joy
  Doom Days (This Got Out of Hand Edition)
    Admit Defeat
    Good Lesson
    Final Hour
    Comfort of Strangers
    Hangin'
  Wild World (Complete Edition)
    Good Grief
    The Currents
    An Act of Kindness
    Warmth
    Glory
    Power
    Two Evils
    Send Them Off!
    Lethargy
    Blame
    Fake It
    Snakes
    Winter of Our Youth
    Way Beyond
    Oil On Water
    Campus
    Shame
    The Anchor
```
