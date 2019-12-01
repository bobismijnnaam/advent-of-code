total = 0

with open("input") as f:
	for line in f:
		dims = line.split("x")
		l = int(dims[0])
		w = int(dims[1])
		h = int(dims[2])

		area = 2 * l * w + 2 * w * h + 2 * h * l
		extra = min(l * w, w * h, h * l)

		total += area + extra

print total

total = 0
with open("input2") as f:
	for line in f:
		dims = [int(x) for x in line.split("x")]
		l = dims[0]
		w = dims[1]
		h = dims[2]
	
		bow = l * w * h
		ribbon = sum(sorted(dims)[0:2]) * 2
		
		total += bow + ribbon

print total
