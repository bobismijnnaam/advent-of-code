from itertools import chain

def simulateDay(fish):
    assert 0 <= fish and fish <= 8

    if fish == 0:
        return [8, 6]
    else:
        return [fish - 1]

def emptyPool():
    fishPool = {}
    for i in range(8 + 1):
        fishPool[i] = 0
    return fishPool

def simulatePool(fishPool):
    newFishpool = emptyPool()
    for i in range(1, 8 + 1):
        newFishpool[i - 1] = fishPool[i]
    newFishpool[6] += fishPool[0]
    newFishpool[8] += fishPool[0]
    return newFishpool


def main():
    with open("input.txt", "r") as f:
        inputFish = [int(x.strip()) for x in f.readline().split(",")]

    fish = inputFish

    print("start:", fish)

    for i in range(80):
        newFish = list(chain.from_iterable(simulateDay(aFish) for aFish in fish))
        fish = newFish
    
    print("num fish:", len(fish))

    fishPool = emptyPool()
    for aFish in inputFish:
        fishPool[aFish] += 1
    
    print("fishPool", fishPool)
    for i in range(80):
        fishPool = simulatePool(fishPool)
        print(fishPool)

    print("num fish:", sum(fishPool.values()))

    fishPool = emptyPool()
    for aFish in inputFish:
        fishPool[aFish] += 1
    
    print("fishPool", fishPool)
    for i in range(256):
        fishPool = simulatePool(fishPool)
        print(fishPool)

    print("num fish:", sum(fishPool.values()))
    
if __name__ == "__main__":
    main()
