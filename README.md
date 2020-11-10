# MusicScraper
Python scipt to get ID3 tags and output them

Based on the original script made by my friend, [MMaker](https://github.com/stysmmaker)

## Dependencies
[pytaglib](https://pypi.python.org/pypi/pytaglib)
[libtag1-dev](https://taglib.org/)

## Usage
``python main.py``

You may specify an argument which directs to your music collection.
This is usually in /home/{$USER}/Music.
However, placing the path as the argument will override the default folder path.

# Example Output
This was created from m4a files. According to pytaglib, it should support other formats, including:
FLAC, MPC, Speex, WavPack, TrueAudio, WAV, AIFF, mp4 and ASF files.

Best results come from an iTunes library, for example, as it will have all the necessary id3 tags.

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
