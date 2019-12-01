import re
import time

setRe = re.compile("(\d{1,5}) -> (\w+)")
copyRe = re.compile("(\w+) -> (\w+)")
andRe = re.compile("(\w+) AND (\w+) -> (\w+)")
orRe = re.compile("(\w+) OR (\w+) -> (\w+)")
lshiftRe = re.compile("(\w+) LSHIFT (\d{1,2}) -> (\w+)")
rshiftRe = re.compile("(\w+) RSHIFT (\d{1,2}) -> (\w+)")
notRe = re.compile("NOT (\w+) -> (\w+)")

outputs = {}
outputsSet = 0

with open('input') as f:
	lines = f.readlines()

def handleLine(line):
	global outputsSet
	global outputs

	setMatch = setRe.match(line)
	copyMatch = copyRe.match(line)
	andMatch = andRe.match(line)
	orMatch = orRe.match(line)
	lshiftMatch = lshiftRe.match(line)
	rshiftMatch = rshiftRe.match(line)
	notMatch = notRe.match(line)
	
	if setMatch:
		num = int(setMatch.group(1))
		name = setMatch.group(2)
		if not name in outputs:
			outputs[name] = num
			outputsSet += 1
	elif copyMatch:
		src = copyMatch.group(1)
		dst = copyMatch.group(2)
		if src in outputs and not dst in outputs:
			outputs[dst] = outputs[src]
			outputsSet += 1
	elif andMatch:
		left = andMatch.group(1)
		right = andMatch.group(2)
		name = andMatch.group(3)
		
		if left.isdigit():
			leftVal = int(left)
		elif left in outputs:
			leftVal = outputs[left]
		else: return
		
		if right.isdigit():
			rightVal = int(right)
		elif right in outputs:
			rightVal = outputs[right]
		else: return

		if not name in outputs:
			outputs[name] = leftVal & rightVal
			outputsSet += 1
	elif orMatch:
		left = orMatch.group(1)
		right = orMatch.group(2)
		name = orMatch.group(3)

		if left.isdigit():
			leftVal = int(left)
		elif left in outputs:
			leftVal = outputs[left]
		else: return
		
		if right.isdigit():
			rightVal = int(right)
		elif right in outputs:
			rightVal = outputs[right]
		else: return

		if not name in outputs:
			outputs[name] = leftVal | rightVal
			outputsSet += 1
	elif lshiftMatch:
		left = lshiftMatch.group(1)
		num = int(lshiftMatch.group(2))
		name = lshiftMatch.group(3)
		if left in outputs and not name in outputs:
			outputs[name] = outputs[left] << num
			outputsSet += 1
	elif rshiftMatch:
		left = rshiftMatch.group(1)
		num = int(rshiftMatch.group(2))
		name = rshiftMatch.group(3)
		if left in outputs and not name in outputs:
			outputs[name] = outputs[left] >> num
			outputsSet += 1
	elif notMatch:
		left = notMatch.group(1)
		name = notMatch.group(2)
		if left in outputs and not name in outputs:
			outputs[name] = ~outputs[left]
			outputsSet += 1

while not outputsSet == len(lines):
	for line in lines:
		handleLine(line)

for line in lines:
	handleLine(line)

print outputs['a']
		
lines.remove("44430 -> b\n")
lines += ["3176 -> b"]

outputsSet = 0
outputs = {}

while not outputsSet == len(lines):
	for line in lines:
		handleLine(line)

for line in lines:
	handleLine(line)

print outputs['a']
