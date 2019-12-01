import re

total = 0

with open('input') as f:
	for linex in f:
		line = linex.replace("\n", "")
		print line

		total += len(line)
		
		inMem = line[1:-1]
		print inMem
		# inMem -= line.count("\\\\")
		inMem = inMem.replace("\\\\", "B")
		# inMem -= line.count("\\\"")
		inMem = inMem.replace("\\\"", "A")
		# inMem -= line.count("\\x") * 3
		inMem = re.sub("\\\\x\w\w", "X", inMem)
		
		total -= len(inMem)

print total

total = 0

with open('input') as f:
	for linex in f:
		line = linex.replace("\n", "")
		total -= len(line)

		inMem = line
		inMem = inMem.replace("\\", "\\\\")
		inMem = inMem.replace("\"", "\\\"")

		inMem = "\"" + inMem + "\""

		total += len(inMem)

print total
		
