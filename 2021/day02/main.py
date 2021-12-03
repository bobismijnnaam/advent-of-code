def parse(line):
    splits = line.split(" ")
    return (splits[0], int(splits[1]))

FORWARD = "forward"
DOWN = "down"
UP = "up"

with open("input.txt", "r") as f:
    cmds = [parse(line) for line in f.readlines()]

print(cmds)

def runCommands(cmds):
    horizontal = 0
    depth = 0
    for cmd in cmds:
        if cmd[0] == FORWARD:
            horizontal += cmd[1]
        elif cmd[0] == DOWN:
            depth += cmd[1]
        elif cmd[0] == UP:
            depth -= cmd[1]
        else:
            print("wrong:", cmd)
    return (horizontal, depth)

(hor, dep) = runCommands(cmds)
print(hor * dep)

def runCommandsAim(cmds):
    horizontal = 0
    depth = 0
    aim = 0
    for cmd in cmds:
        if cmd[0] == FORWARD:
            horizontal += cmd[1]
            depth += aim * cmd[1]
        elif cmd[0] == DOWN:
            aim += cmd[1]
        elif cmd[0] == UP:
            aim -= cmd[1]
        else:
            print("wrong:", cmd)
    return (horizontal, depth)

(hor, dep) = runCommandsAim(cmds)
print(hor * dep)
