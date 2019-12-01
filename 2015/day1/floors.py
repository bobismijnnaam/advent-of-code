with open("input") as f:
	contents = f.read()

floor = 0
trigger = None

for i, c in enumerate(contents):
	if c == '(':
		floor += 1
	elif c == ')':
		floor -= 1

	if floor == -1 and trigger == None:
		trigger = i + 1

print floor
print trigger
