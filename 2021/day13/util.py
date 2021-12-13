class Vec2:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __add__(self, other):
        return Vec2(self.x + other.x, self.y + other.y)
    
    def __sub__(self, other):
        return Vec2(self.x - other.x, self.y - other.y)

    def rot90(self):
        return Vec2(-self.y, self.x)

    def __repr__(self):
        return f"Vec2({self.x}, {self.y})"

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __hash__(self):
        return hash((self.x, self.y))

RIGHT = Vec2(1, 0)
BOTTOM = DOWN = RIGHT.rot90()
LEFT = DOWN.rot90()
TOP = UP = LEFT.rot90()
NEIGHBOURS4 = [RIGHT, DOWN, LEFT, UP]

TOP_RIGHT = Vec2(1, 1)
TOP_LEFT = TOP_RIGHT.rot90()
BOTTOM_LEFT = TOP_LEFT.rot90()
BOTTOM_RIGHT = BOTTOM_LEFT.rot90()
NEIGHBOURS8 = [RIGHT, TOP_RIGHT, TOP, TOP_LEFT, LEFT, BOTTOM_LEFT, BOTTOM, BOTTOM_RIGHT]

def neighbours4(p):
    return [p + d for d in NEIGHBOURS4]

def neighbours8(p):
    return [p + d for d in NEIGHBOURS8]

def grid(p):
    for y in range(p.y):
        for x in range(p.x):
            yield Vec2(x, y)
