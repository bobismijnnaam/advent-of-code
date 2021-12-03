with open("input.txt", "r") as f:
# with open("test.txt", "r") as f:
    nums = [[int(i) for i in line.strip()] for line in f.readlines()]

gamma = []

EQUAL = -1

def mostCommonInPos(nums, pos):
    count = [0, 0]
    for num in nums:
        count[num[pos]] += 1
    if count[0] > count[1]:
        return 0
    elif count[1] > count[0]:
        return 1
    else:
        return EQUAL

for i in range(len(nums[0])):
    gamma += [mostCommonInPos(nums, i)]

print(gamma)

epsilon = [1 - i for i in gamma]

print(epsilon)

def binListToInt(binList):
    return int("".join(str(i) for i in binList), 2)

gammaInt = binListToInt(gamma)
epsilonInt = binListToInt(epsilon)

power = gammaInt * epsilonInt
print(power)

def iterateBitCriteria(nums, pos, preference):
    print("-----")
    print("pos", pos)
    print("pref", preference)
    print(nums)
    if len(nums) == 1: return nums[0]
    if len(nums) == 0: raise Exception()

    mostCommon = mostCommonInPos(nums, pos)
    print("mostCommon", mostCommon)
    if mostCommon == EQUAL:
        selector = preference
    elif preference == 0:
        # Make least common
        selector = 1 - mostCommon
    else:
        selector = mostCommon
    print("selector", selector)

    newNums = [num for num in nums if num[pos] == selector]
    return iterateBitCriteria(newNums, pos + 1, preference)

print("oxygen")
oxygen = iterateBitCriteria(nums, 0, 1)
print("co2")
co2scrubber = iterateBitCriteria(nums, 0, 0)

print(oxygen, co2scrubber)

print(binListToInt(oxygen), binListToInt(co2scrubber))
print(binListToInt(oxygen) * binListToInt(co2scrubber))


