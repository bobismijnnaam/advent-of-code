from collections import defaultdict

def small(n):
    return n.islower()

def big(n):
    return n.isupper()

def r(G, n, currentPath):
    if n == "end":
        p = currentPath + (n,)
        return set([p])
    if n == "start" and n in currentPath:
        return set()

    if small(n):
        if n in currentPath:
            return set()

    currentPath += (n,)

    allPaths = set()
    for m in G[n]:
        allPaths.update(r(G, m, currentPath))

    return allPaths

def countSmallOccurrences(path):
    if len(path) == 0: raise Exception()
    return max(path.count(n) for n in path if small(n))

def r2(G, n, currentPath):
    if n == "end":
        p = currentPath + (n,)
        return set([p])
    elif n == "start":
        if n in currentPath:
            return set()
    elif small(n):
        if n in currentPath:
            if countSmallOccurrences(currentPath) >= 2:
                return set()

    currentPath += (n,)

    allPaths = set()
    for m in G[n]:
        allPaths.update(r2(G, m, currentPath))

    return allPaths


def main(inputPath):
    print("Considering", inputPath)

    with open(inputPath, "r") as f:
        edgePairs = [line.strip().split("-") for line in f.readlines()]

    # print(edgePairs)

    edges = defaultdict(set)
    for f, t in edgePairs:
        edges[f].add(t)
        edges[t].add(f)

    # print(edges)
    # print(r(edges, "start", ()))
    print(len(r(edges, "start", ())))

    # for p in sorted(r2(edges, "start", ())):
    #     print(p)

    print(len(r2(edges, "start", ())))

if __name__ == "__main__":
    main("test0.txt")
    main("test1.txt")
    main("test2.txt")
    main("input.txt")
    print("Ok.")
