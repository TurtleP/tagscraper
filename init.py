import sys
import os
from datetime import datetime

from music import Music

try:
	musicDir = sys.argv[1] 
except IndexError:
	musicDir = "/home/" + os.environ["USER"] + "/Music"

music = dict()
counts = []

for i in range(3):
	counts.append(0)

album_count = dict()
album_year = dict()
for path, dirs, files in os.walk(musicDir):
	for filename in files:
		filePath = os.path.join(path, filename)
		if os.path.isfile(filePath):
			song = Music(filePath)

			if not song.getArtist() in music:
				music[song.getArtist()] = dict()
				
				album_count[song.getArtist()] = 0
				counts[0] = counts[0] + 1
			
			if not song.getAlbum() in music[song.getArtist()]:
				music[song.getArtist()][song.getAlbum()] = []

				album_year[song.getAlbum()] = ""
				album_count[song.getArtist()] = album_count[song.getArtist()] + 1
				album_year[song.getAlbum()] = song.getYear()

				counts[1] = counts[1] + 1

			music[song.getArtist()][song.getAlbum()].append(song.getTitle())
			counts[2] = counts[2] + 1

def songs(songList, album_count, total):
	for i in range(len(songList)):
		if total > 1 and album_count < total:
			if i + 1 == len(songList):
				print(" │  └─ " + songList[i])
			elif i + 1 < len(songList):
				print(" │  ├─ " + songList[i])
		else:
			if i + 1 < len(songList):
				print("    ├─ " + songList[i])
			elif i + 1 == len(songList):
				print("    └─ " + songList[i])

print("Compile Date/Time:")
print(datetime.now().strftime("%Y-%m-%d %H:%M:%S"))
print("")
print("Artists: " + str(counts[0]) + " | Albums: " + str(counts[1]) + " | Songs: " + str(counts[2]))
print("")

for artist in sorted(music.keys()):
	print(artist)
	loop_album_count = 0
	for album in music[artist]:
		loop_album_count = loop_album_count + 1

		if loop_album_count != album_count[artist]:
			print(" ├─ " + album + " " + album_year[album])
		else:
			print(" └─ " + album + " " + album_year[album])
		
		songs(music[artist][album], loop_album_count, album_count[artist])

	print("")