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

def allMappings():
    identityMapping = "abcdefg"
    return [dict(zip(candidateMap, identityMapping)) for candidateMap in itertools.permutations(identityMapping)]

def applyMapping(pat, aMap):
    return sevenSegToInt("".join(aMap[c] for c in pat))

def decodeJob(job):
    (examples, outputs) = job
    for aMap in allMappings():
        decoding = set(applyMapping(pat, aMap) for pat in examples)
        if len(decoding) == 10 and not None in decoding:
            return int("".join(str(applyMapping(pat, aMap)) for pat in outputs))

def sevenSegToInt(pat):
    if set(pat) == set("abcefg"):
        return 0
    if set(pat) == set("cf"):
        return 1
    if set(pat) == set("acdeg"):
        return 2
    if set(pat) == set("acdfg"):
        return 3
    if set(pat) == set("bdcf"):
        return 4
    if set(pat) == set("abdfg"):
        return 5
    if set(pat) == set("abdefg"):
        return 6
    if set(pat) == set("acf"):
        return 7
    if set(pat) == set("abcdefg"):
        return 8
    if set(pat) == set("abcdfg"):
        return 9
    return None

def main():
    with open("input.txt") as f:
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
    total = 0
    for job in jobs:
        v = decodeJob(job)
        print(v)
        total += v
    print(total)

if __name__ == "__main__":
    main()

