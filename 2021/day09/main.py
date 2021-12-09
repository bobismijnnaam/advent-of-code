import functools
import operator

from util import *

def main():
    with open("input.txt", "r") as f:
        lines = f.readlines()

    print(RIGHT, DOWN, LEFT, UP)

    heights = {}
    for y, line in enumerate(lines):
        for x, c in enumerate(line.strip()):
            heights[Vec2(x, y)] = int(c)

    width = x + 1
    height = y + 1

    print(heights)
    print(width)
    print(height)

    totalRisk = 0
    lowPoints = []
    for y in range(height):
        for x in range(width):
            pos = Vec2(x, y)
            currentHeight = heights[pos]
            neighbourHeights = list(filter(lambda p: p != None, (heights.get(pos + v) for v in NEIGHBOURS)))
            if all(neighbourHeight > currentHeight for neighbourHeight in neighbourHeights):
                print("-----")
                print(pos)
                print(list(pos + v for v in NEIGHBOURS))
                print(neighbourHeights)
                print(pos, currentHeight)
                totalRisk += currentHeight + 1
                lowPoints += [pos]
    
    print(totalRisk)
    
    basins = []
    for lowPos in lowPoints:
        basin = set([lowPos])
        frontier = set([lowPos])
        while len(frontier) > 0:
            current = frontier.pop()
            neighbours = list(current + v for v in NEIGHBOURS if current + v in heights)
            for neighbour in neighbours:
                if heights[neighbour] < 9 and not neighbour in basin:
                    basin.add(neighbour)
                    frontier.add(neighbour)
        print(lowPos, len(basin))
        basins += [basin]

    basinsLen = list(len(b) for b in basins)
    basinsLen = sorted(basinsLen)
    biggest = basinsLen[-3:]
    print(functools.reduce(operator.mul, biggest))

    # 20400, too low

if __name__ == "__main__":
    main()
