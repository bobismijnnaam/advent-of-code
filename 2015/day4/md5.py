import hashlib

seed = "ckczppom"

md5seed = hashlib.md5()
md5seed.update(seed)

for i in range(1000000):
	md5 = md5seed.copy()
	md5.update(str(i))
	if md5.hexdigest()[0:5] == "00000":
		print str(i)
		break

for i in range(10000000):
	md5 = md5seed.copy()
	md5.update(str(i))
	if md5.hexdigest()[0:6] == "000000":
		print str(i)
		break
