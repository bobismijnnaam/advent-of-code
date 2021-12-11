from util import *

def step(field, fieldSize):
    oldField = field
    field = { p: v + 1 for p, v in field.items() }

    numFlashes = 0
    flashed = set()
    while oldField != field:
        oldField = field.copy()
        for p in grid(fieldSize):
            if field[p] > 9 and p not in flashed:
                print("flash:", p, "val:", field[p])
                flashed.add(p)
                numFlashes += 1
                for n in neighbours8(p):
                    if n in field:
                        field[n] += 1

    for p in flashed:
        field[p] = 0

    return field, numFlashes, flashed

def main():
    with open("input.txt", "r") as f:
        lines = f.readlines()

    field = {}
    for y, line in enumerate(lines):
        for x, c in enumerate(line.strip()):
            field[Vec2(x, y)] = int(c)

    fieldSize = Vec2(x + 1, y + 1)

    print(neighbours8(Vec2(0, 0)))

    originalField = field.copy()
    print("field:", field)

    numFlashes = 0
    for i in range(100):
        field, additionalFlashes, _ = step(field, fieldSize)
        numFlashes += additionalFlashes
        print("field:", field)
        print("numFlashes:", numFlashes)

    field = originalField.copy()
    synchronousFlash = False
    i = 0
    while not synchronousFlash:
        field, _, flashed = step(field, fieldSize)
        i += 1
        print(i)
        if flashed == set(grid(fieldSize)):
            print("synchronous flash:", i)
            break

if __name__ == "__main__":
    main()
