#!/usr/bin/env python3

import datetime
import subprocess
import sys
import os

day = datetime.datetime.now().day

print(day)

command = ["cargo", "init", "day" + str(day)]
print(command)
res = subprocess.run(command)
print(res.returncode)

if res.returncode != 0:
    print("could not create cargo package")

command = ["cp", "day1/.gitignore", "day{}/.gitignore".format(day)]
subprocess.run(command)

from aocd.models import Puzzle
puzzle = Puzzle(year=2019, day=day)

with open("day{}/input.txt".format(day), "w") as f:
    f.write(puzzle.input_data)

pid = os.fork()
if pid == 0: # New process
    subprocess.run(["intellij-idea-ultimate", "day{}".format(day)])
