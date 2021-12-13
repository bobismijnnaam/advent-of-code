from collections import defaultdict
from util import *

def printGrid(grid):
    first = True
    for y in range(max(p.y for p in grid) + 1):
        if not first: print()
        first = False
        for x in range(max(p.x for p in grid) + 1):
            if Vec2(x, y) in grid:
                print("#", end="")
            else:
                print(".", end="")
    print()

def foldY(grid, y):
    newGrid = set()
    for p in grid:
        if p.y < y:
            newGrid.add(p)
        else:
            newGrid.add(Vec2(p.x, y - (p.y - y)))
    return newGrid

def foldX(grid, x):
    newGrid = set()
    for p in grid:
        if p.x < x:
            newGrid.add(p)
        else:
            newGrid.add(Vec2(x - (p.x - x), p.y))
    return newGrid

def main(inputPath):
    print("Considering", inputPath)

    grid = set()
    instrs = []
    with open(inputPath, "r") as f:
        for line in f.readlines():
            line = line.strip()
            if "," in line:
                tup = line.split(",")
                grid.add(Vec2(int(tup[0]), int(tup[1])))
            elif "=" in line:
                if "x" in line:
                    instrs += [("x", int(line.split("=")[1]))]
                elif "y" in line:
                    instrs += [("y", int(line.split("=")[1]))]
                else:
                    raise Exception()
            else:
                print("Skipping:", line)

    print(grid)
    print(instrs)
    printGrid(grid)

    actions = {"x": foldX, "y": foldY}

    originalGrid = grid

    for t, d in instrs:
        grid = actions[t](grid, d)
        printGrid(grid)
        print(len(grid))
        break

    grid = originalGrid

    for t, d in instrs:
        grid = actions[t](grid, d)
        printGrid(grid)

    # ZUJUAFHP
    print(len(grid))

if __name__ == "__main__":
    main("test0.txt")
    # main("test1.txt")
    # main("test2.txt")
    main("input.txt")
    print("Ok.")
