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
DOWN = RIGHT.rot90()
LEFT = DOWN.rot90()
UP = LEFT.rot90()
NEIGHBOURS = [RIGHT, DOWN, LEFT, UP]
