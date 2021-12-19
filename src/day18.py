class Pair:
    def __init__(self, left, right, level=0):
        self.level = level
        if isinstance(left, int):
            self.left = left
        else:
            self.left = Pair(left[0], left[1], level=level + 1)
        if isinstance(right, int):
            self.right = right
        else:
            self.right = Pair(right[0], right[1], level=level + 1)

    def __add__(self, other: "Pair") -> "Pair":
        return Pair(left=self.increase_level(), right=other.increase_level())

    def __repr__(self) -> str:
        return f"Pair(level={self.level!r}, left={self.left!r}, right={self.right!r})"

    def increase_level(self) -> "Pair":
        self.level += 1
        if isinstance(self.left, Pair):
            self.left.increase_level()
        if isinstance(self.right, Pair):
            self.right.increase_level()
        return self

    @classmethod
    def from_list(cls, l: list) -> "Pair":
        return Pair(l[0], l[1])

    def __iter__(self):
        yield from (self.left, self.right)
