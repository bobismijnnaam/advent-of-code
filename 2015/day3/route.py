from collections import Counter

with open("input") as f:
	route = f.read()

deliveries = Counter()
loc = (0, 0)

deliveries[loc] += 1

for d in route:
	if d == ">":
		loc = (loc[0] + 1, loc[1])
	elif d == "<":
		loc = (loc[0] - 1, loc[1])
	elif d == "^":
		loc = (loc[0], loc[1] + 1)
	elif d == "v":
		loc = (loc[0], loc[1] - 1)
	
	deliveries[loc] += 1

print len(deliveries)

deliveries = Counter()
santaLoc = (0, 0)
roboLoc = (0, 0)

deliveries[santaLoc] += 1
deliveries[roboLoc] += 1

turn = "santa"

for d in route:
	if turn == "santa":
		loc = santaLoc
	else:
		loc = roboLoc

	if d == ">":
		loc = (loc[0] + 1, loc[1])
	elif d == "<":
		loc = (loc[0] - 1, loc[1])
	elif d == "^":
		loc = (loc[0], loc[1] + 1)
	elif d == "v":
		loc = (loc[0], loc[1] - 1)
	
	deliveries[loc] += 1

	if turn == "santa":
		santaLoc = loc
		turn = "robo"
	else:
		roboLoc = loc
		turn = "santa"

print len(deliveries)
		
