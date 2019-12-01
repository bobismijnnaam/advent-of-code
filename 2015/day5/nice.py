from collections import Counter

with open("input") as f:
	strings = f.readlines()

niceStringCount = 0
fiends = ["ab", "cd", "pq", "xy"]

for string in strings:
	letterSet = Counter(string)
	property1 = letterSet['a'] + letterSet['e'] + letterSet['i'] + letterSet['o'] + letterSet['u'] > 2

	property2 = False
	for i, letter in enumerate(string[1:]):
		i = i + 1
		if letter == string[i - 1]:
			property2 = True
			break
	
	property3 = True
	for fiend in fiends:
		if fiend in string:
			property3 = False
			break
	
	if property1 and property2 and property3:
		niceStringCount += 1

print niceStringCount

niceStringCount = 0

for string in strings:
	property1 = False

	for i, firstLetter in enumerate(string[:-3]):
		pair = firstLetter + string[i + 1]

		for j in range(i + 2, len(string) - 1):
			otherPair = string[j] + string[j + 1]
			if pair == otherPair:
				property1 = True
				break

	property2 = False
	
	for i, letter in enumerate(string[:-2]):
		if letter == string[i + 2]:
			property2 = True
			break

	if property1 and property2:
		niceStringCount += 1

print niceStringCount
	

