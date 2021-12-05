import re
from collections import Counter

RE = re.compile("(\d+),(\d+) -> (\d+),(\d+)")

def v2add(a, b):
    return (a[0] + b[0], a[1] + b[1])

def sign(a):
    if a > 0:
        return 1
    elif a < 0:
        return -1
    else:
        return 0

def isHorOrVert(line):
    return line[0] == line[2] or line[1] == line[3]

def expandLine(line):
    print(line)
    start = (line[0], line[1])
    end = (line[2], line[3])

    diffX = sign(end[0] - start[0])
    diffY = sign(end[1] - start[1])

    diff = (diffX, diffY)

    curr = v2add(start, diff)
    positions = [start]
    while curr != end:
        positions += [curr]
        curr = v2add(curr, diff)
    positions += [end]

    return positions

def main():
    with open("input.txt", "r") as f:
        inputLines = f.readlines()

    lines = [[int(x) for x in RE.match(line).groups()] for line in inputLines]

    print(lines)

    positions = Counter()
    for line in lines:
        if isHorOrVert(line):
            linePositions = expandLine(line)
            print(linePositions)
            for position in linePositions:
                positions[position] += 1

    dangerousPositions = [pos for pos, count in positions.items() if count >= 2]
    print("dp:", dangerousPositions)
    print(len(dangerousPositions))

    positions = Counter()
    for line in lines:
        linePositions = expandLine(line)
        print(linePositions)
        for position in linePositions:
            positions[position] += 1

    dangerousPositions = [pos for pos, count in positions.items() if count >= 2]
    print("dp:", dangerousPositions)
    print(len(dangerousPositions))

if __name__ == "__main__":
    main()
