import itertools

def parseLine(line):
    [config, output] = line.split("|")
    return (config.strip().split(" "), output.strip().split(" "))

def guessPattern(pattern):
    if len(pattern) == 2:
        return 1
    elif len(pattern) == 4:
        return 4
    elif len(pattern) == 3:
        return 7
    elif len(pattern) == 7:
        return 8
    else:
        return None

def guessSingleMapping(pattern):
    if len(pattern) == 2: # 1
        return "cf"
    elif len(pattern) == 4: # 4
        return "bdcf"
    elif len(pattern) == 3: # 7
        return "dab"
    else:
        return None

def guessMappingWithPermute(pattern):
    guess = guessSingleMapping(pattern)
    if guess:
        allMaps = itertools.permutations(guess)
        return set(dict(zip(pat, aMap)) for aMap in allMaps)
    else:
        return set()

def allMappings():
    identityMapping = "abcdefg"
    return [dict(zip(candidateMap, identityMapping)) for candidateMap in itertools.permutations(identityMapping)]

def applyMapping(pat, aMap):
    return int("".join(aMap[c] for c in pat))

def decodeJob(job):
    (examples, outputs) = job
    for aMap in allMappings():
        decoding = set(applyMapping(pat, aMap) for pat in examples)
        if len(decoding) == 10:
            return int("".join(applyMapping(pat) for pat in outputs))

def sevenSegToInt(pat):
    if set(pat) == set("

def main():
    with open("test.txt") as f:
        job = [parseLine(line) for line in f.readlines()]

    print(job)

    total = 0
    for (_, outputs) in job:
        total += sum(1 for output in outputs if guessPattern(output))

    print(total)

    doesNotHaveEight = 0
    for (examples, outputs) in job:
        together = examples + outputs
        if not any(guessPattern(pat) == 8 for pat in together):
            doesNotHaveEight += 1

    print("doesNotHaveEight:", doesNotHaveEight)

    doesNotHaveFive = 0
    for (examples, outputs) in job:
        together = examples + outputs
        if len(set(guessPattern(pat) for pat in together)) != 5:
            doesNotHaveFive += 1

    print("doesNotHaveFive:", doesNotHaveFive)

    print(len(allMappings()))

    jobs = job
    for job in jobs:
        print(decodeJob(job))

if __name__ == "__main__":
    main()

