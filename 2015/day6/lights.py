import re
from collections import Counter

onRe = re.compile("turn on (\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})")
toggleRe = re.compile("toggle (\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})")
offRe = re.compile("turn off (\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})")

with open("input") as f:
	lines = f.readlines()

lights = set()

def extract(mo):
	return int(mo.group(1)), int(mo.group(2)), int(mo.group(3)), int(mo.group(4))

for line in lines:
	onMatch = onRe.match(line)
	toggleMatch = toggleRe.match(line)
	offMatch = offRe.match(line)
	
	if onMatch:
		x1, y1, x2, y2 = extract(onMatch)
		for x in range(x1, x2 + 1):
			for y in range(y1, y2 + 1):
				lights.add((x, y))
	elif toggleMatch:
		x1, y1, x2, y2 = extract(toggleMatch)
		for x in range(x1, x2 + 1):
			for y in range(y1, y2 + 1):
				if (x, y) in lights:
					lights.discard((x, y))
				else:
					lights.add((x, y))
	elif offMatch:
		x1, y1, x2, y2 = extract(offMatch)
		for x in range(x1, x2 + 1):
			for y in range(y1, y2 + 1):
				lights.discard((x, y))

print len(lights)

lights = Counter()

for line in lines:
	onMatch = onRe.match(line)
	toggleMatch = toggleRe.match(line)
	offMatch = offRe.match(line)
	
	if onMatch:
		x1, y1, x2, y2 = extract(onMatch)
		for x in range(x1, x2 + 1):
			for y in range(y1, y2 + 1):
				lights[(x, y)] += 1
	elif toggleMatch:
		x1, y1, x2, y2 = extract(toggleMatch)
		for x in range(x1, x2 + 1):
			for y in range(y1, y2 + 1):
				lights[(x, y)] += 2
	elif offMatch:
		x1, y1, x2, y2 = extract(offMatch)
		for x in range(x1, x2 + 1):
			for y in range(y1, y2 + 1):
				if lights[(x, y)] > 0:
					lights[(x, y)] -= 1

print sum(lights.values())
