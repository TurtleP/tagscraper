import taglib

class Music:
	def __init__(self, name):
		self.tags = taglib.File(name).tags

	def getArtist(self):
		return self.tags["ALBUMARTIST"][0].title()

	def getAlbum(self):
		return self.tags["ALBUM"][0]

	def getTitle(self):
		return self.tags["TITLE"][0]
	
	def getYear(self):
		return "(" + self.tags["DATE"][0] + ")"