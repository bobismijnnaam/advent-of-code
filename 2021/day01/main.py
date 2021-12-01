
with open("input.txt", "r") as f:
    xs = [int(x) for x in f.readlines()]

print(xs)

prev = xs[0]
larger = 0
for x in xs:
    if x > prev:
        larger = larger + 1
    prev = x

print(larger)

triplets = [[x, x + 1, x + 2] for x in range(len(xs))]
triplets = [triplet for triplet in triplets if all(0 <= i and i < len(xs) for i in triplet)]

print(triplets)

triplets = [sum(xs[i] for i in triplet) for triplet in triplets]

prev = triplets[0]
larger = 0
for x in triplets:
    if x > prev:
        larger = larger + 1
    prev = x
print(larger)
