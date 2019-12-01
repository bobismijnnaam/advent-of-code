from itertools import permutations
import re
from collections import defaultdict

lineRe = re.compile("(\w*) to (\w*) = (\d*)")
distances = defaultdict(dict)

for line in open("input", "r"):
    match = lineRe.match(line)
    src = match.group(1)
    dst = match.group(2)
    dist = int(match.group(3))
    
    distances[src][dst] = dist
    distances[dst][src] = dist

places = distances.keys()
minDist = None
for trip in permutations(places):
    dist = 0
    for i, place in enumerate(trip[:-1]):
        dist += distances[place][trip[i+1]]

    if minDist:
        minDist = min(dist, minDist)
    else:
        minDist = dist

print minDist

places = distances.keys()
minDist = None
for trip in permutations(places):
    dist = 0
    for i, place in enumerate(trip[:-1]):
        dist += distances[place][trip[i+1]]

    if minDist:
        minDist = max(dist, minDist)
    else:
        minDist = dist

print minDist
