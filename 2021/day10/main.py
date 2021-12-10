PAIRS = {"(": ")", "[": "]", "{": "}", "<": ">"}
POINTS = {")": 3, "]": 57, "}": 1197, ">": 25137}
AUTO_POINTS = { ")": 1, "]": 2, "}": 3, ">": 4 }

def main():
    with open("input.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]

    total = 0
    incomplete = []
    for line in lines:
        stack = []
        for c in line:
            if c in PAIRS:
                stack += [c]
            else:
                if PAIRS[stack[-1]] != c:
                    print("unbalanced: ", c)
                    total += POINTS[c]
                    stack = []
                    break
                else:
                    stack.pop()
        if stack:
            incomplete += [stack]

    print(len(incomplete))
    totals = []
    for completion in incomplete:
        print("---")
        print("".join(completion))
        print("".join(PAIRS[c] for c in completion[::-1]))
        total = 0
        for c in completion[::-1]:
            total = total * 5
            total += AUTO_POINTS[PAIRS[c]]
        print(total)
        totals += [total]


    # import statistics
    # print(statistics.median(totals))

    # 1596670368, too low
    print("---")
    totals = sorted(totals)
    print(totals)
    print(totals[(len(totals) // 2)])


if __name__ == "__main__":
    main()
