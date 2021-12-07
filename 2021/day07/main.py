def costForPos(nums, targetPos):
    return sum(abs(pos - targetPos) for pos in nums)

def sumUntil(n):
    if n == 0:
        return 0
    elif n == 1:
        return 1
    else:
        return (n * (n + 1)) // 2

def costForPosSum(nums, targetPos):
    return sum(sumUntil(abs(pos - targetPos)) for pos in nums)

def main():
    with open("input.txt", "r") as f:
        nums = [int(x) for x in f.readline().split(",")]

    minPos = min(nums)
    maxPos = max(nums)

    minPosSeen = minPos
    minCostSeen = costForPos(nums, minPos)
    for p in range(minPos, maxPos + 1):
        cost = costForPos(nums, p)
        if cost < minCostSeen:
            minCostSeen = cost
            minPosSeen = p

    print(minCostSeen)
    import statistics
    print(minPosSeen, "median:", statistics.median(nums))

    minPos = min(nums)
    maxPos = max(nums)

    minPosSeen = minPos
    minCostSeen = costForPosSum(nums, minPos)
    for p in range(minPos, maxPos + 1):
        cost = costForPosSum(nums, p)
        if cost < minCostSeen:
            minCostSeen = cost
            minPosSeen = p

    # 92772307, too low
    print(minCostSeen)
    print(minPosSeen, statistics.mean(nums))

if __name__ == "__main__":
    main()



