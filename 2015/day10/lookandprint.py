import re

contents = "1113122113"

print contents

for i in range(40):
    print "iter ", i
    newContents = ""
    while contents:
        head = contents[0]
        headLen = len(re.match("%s+" % head, contents).group())
        newContents += "%s%s" % (headLen, head)
        contents = contents[headLen:]
    contents = newContents

print len(contents)

contents = "1113122113"

reDict = {
    "1": re.compile("1+"),
    "2": re.compile("2+"),
    "3": re.compile("3+")
}

for i in range(50):
    print "iter ", i
    newContents = ""
    while contents:
        head = contents[0]
        headLen = len(reDict[head].match(contents).group())
        newContents += "%s%s" % (headLen, head)
        contents = contents[headLen:]
    contents = newContents

print len(contents)
