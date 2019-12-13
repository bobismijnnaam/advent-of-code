#!/usr/bin/env python3

import datetime
import subprocess
import sys
import os

day = datetime.datetime.now().day

print("Puzzle", day)

print("Creating cargo pkg...")
command = ["cargo", "init", "day" + str(day)]
res = subprocess.run(command)

if res.returncode != 0:
    print("could not create cargo package")

print("Copying .gitignore...")
command = ["cp", "day1/.gitignore", "day{}/.gitignore".format(day)]
subprocess.run(command)

print("Copying util...")
command = ["cp", "-r", "day{}/src/util".format(day-1), "day{}/src".format(day)]
subprocess.run(command)

print("Adding itertools...")
rel_path = os.path.join(os.path.abspath(sys.path[0]), "day{}".format(day))
command = ["cargo", "add", "itertools"]
subprocess.run(command, cwd=rel_path)

print("Adding nalgebra...")
rel_path = os.path.join(os.path.abspath(sys.path[0]), "day{}".format(day))
command = ["cargo", "add", "nalgebra"]
subprocess.run(command, cwd=rel_path)

print("Downloading puzzle...")
from aocd.models import Puzzle
puzzle = Puzzle(year=2019, day=day)

with open("day{}/input.txt".format(day), "w") as f:
    f.write(puzzle.input_data)

print("Starting intellij")
pid = os.fork()
if pid == 0: # New process
    subprocess.run(["intellij-idea-ultimate", "day{}".format(day)])
