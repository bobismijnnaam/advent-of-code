import re
import itertools
import functools
import operator

fieldSize = 5

def toRows(card):
    return card

def toCols(card):
    cols = []
    for i in range(fieldSize):
        cols += [ [row[i] for row in card] ]
    return cols

def isWinnerSeq(rowOrCol, drawn):
    return all(num in drawn for num in rowOrCol)

def wins(card, drawns):
    rows = toRows(card)
    cols = toCols(card)

    return (any(isWinnerSeq(row, drawns) for row in rows)
        or any(isWinnerSeq(col, drawns) for col in cols))
        
def sumUnmarked(card, drawns):
    allNums = itertools.chain(*card)
    allNonDrawnNums = [x for x in allNums if x not in drawns]
    return functools.reduce(operator.add, allNonDrawnNums)

def howManyDrawsNeeded(allDraws, card):
    for i in range(1, len(allDraws)):
        currentDrawns = allDraws[:i]
        if wins(card, allDraws[:i]):
            return i

    raise Exception("Cannot win card with given draws")

def main():
    with open("input.txt", "r") as f:
        lines = f.readlines()

    drawns = [int(x) for x in lines[0].split(",")]
    print(drawns)

    lines = lines[1:]

    print(len(lines[0]))

    def parseLine(line):
        return [int(i) for i in re.compile("\s+").split(line.strip())]

    print(parseLine("1 2  3 4  5"))

    cards = []

    for i, line in enumerate(lines):
        if len(line) == 1:
            card = []
            for j in range(i + 1, i + fieldSize + 1):
                card += [parseLine(lines[j])]
            cards += [card]

    leastDrawsSeen = howManyDrawsNeeded(drawns, cards[0])
    leastCard = cards[0]
    for card in cards[1:]:
        numDrawsNeeded = howManyDrawsNeeded(drawns, card)
        if numDrawsNeeded < leastDrawsSeen:
            leastDrawsSeen = numDrawsNeeded
            leastCard = card

    print("leastDrawsSeen:", leastDrawsSeen)
    print("leastCard:", leastCard)
    leastDraws = drawns[:leastDrawsSeen]
    print(sumUnmarked(leastCard, leastDraws) * leastDraws[-1])

    mostDrawsSeen = howManyDrawsNeeded(drawns, cards[0])
    mostCard = cards[0]
    for card in cards[1:]:
        numDrawsNeeded = howManyDrawsNeeded(drawns, card)
        if numDrawsNeeded > mostDrawsSeen:
            mostDrawsSeen = numDrawsNeeded
            mostCard = card

    print("mostDrawsSeen:", mostDrawsSeen)
    print("mostCard:", mostCard)
    mostDraws = drawns[:mostDrawsSeen]
    print(sumUnmarked(mostCard, mostDraws) * mostDraws[-1])

if __name__ == "__main__":
    main()
