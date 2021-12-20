class Pair:
    def __init__(self, left, right, parent=None):
        self.parent = parent
        if self.parent is None:
            self.level = 0
        else:
            self.level = self.parent.level + 1
        if isinstance(left, int):
            self.left = left
        else:
            self.left = Pair(left[0], left[1], parent=self)
        if isinstance(right, int):
            self.right = right
        else:
            self.right = Pair(right[0], right[1], parent=self)

    def __add__(self, other: "Pair") -> "Pair":
        return reduce(Pair(left=self, right=other))

    def __repr__(self) -> str:
        return f"Pair(level={self.level!r}, left={self.left!r}, right={self.right!r})"

    def __str__(self):
        l,r = self
        if isinstance(l, int) and isinstance(r, int):
            return f"[{l}, {r}]"
        if isinstance(l, Pair) and isinstance(r, int):
            return f"[{l!s}, {r}]"
        if isinstance(r, Pair) and isinstance(l, int):
            return f"[{l}, {r!s}]"
        if isinstance(l, Pair) and isinstance(r, Pair):
            return f"[{l!s}, {r!s}]"
        
    @classmethod
    def from_list(cls, l: list) -> "Pair":
        return Pair(l[0], l[1])

    def __iter__(self):
        yield from (self.left, self.right)

    def __getitem__(self, idx):
        if idx == 0:
            return self.left
        elif idx == 1:
            return self.right
        else:
            raise IndexError(f"Wrong inde , got {idx}, expected 0 or 1")

    def __eq__(self, other):
        if not isinstance(other, Pair):
            return False
        if self.level != other.level:
            return False
        if isinstance(self.left, int):
            return isinstance(other.left, int) and self.left == other.left
        if isinstance(self.right, int):
            return isinstance(other.right, int) and self.right == other.right
        return self.left == other.left and self.right == other.right

    @property
    def is_left(self):
        if self.parent is None:
            return False
        l,_ = self.parent
        return l == self

    @property
    def is_right(self):
        if self.parent is None:
            return False
        _,r = self.parent
        return r == self

    def add_parent_left(self, value):
        if self.parent is None:
            return 
        if self.is_left:
            return self.parent.add_parent_left(value)

        if isinstance(self.parent.left, int):
            self.parent.left += value
            return
        # we're on a "right" branch and the left besides is a pair, we need to go down to the right
        to_add = self.parent.left
        while not isinstance(to_add.right, int):
            to_add = to_add.right
        to_add.right += value

    def add_parent_right(self, value):
        if self.parent is None:
            return 
        if self.is_right:
            return self.parent.add_parent_right(value)
        if isinstance(self.parent.right, int):
            self.parent.right += value
            return
        to_add = self.parent.right
        while not isinstance(to_add.left, int):
            to_add = to_add.left
        to_add.left += value

def explode(pair):
    if pair.level == 4:
        l,r = pair
        pair.add_parent_left(l)
        pair.add_parent_right(r)
        if isinstance(pair.parent.left, Pair) and pair.parent.left == pair:
            pair.parent.left = 0
        else:
            pair.parent.right = 0
        return True
    else:
        if isinstance(pair.left, Pair):
            if explode(pair.left):
                return True
        if isinstance(pair.right, Pair):
            if explode(pair.right):
                return True
        return False

def split(pair):
    l,r = pair
    if isinstance(l, int) and l >=10:
        a,b = (l//2, (l+1)//2)
        pair.left = Pair(a,b,parent=pair)
        return True
    if isinstance(r, int) and r >=10:
        a,b = (r//2, (r+1)//2)
        pair.right = Pair(a,b,parent=pair)
        return True
    if isinstance(l, Pair):
        if split(pair.left):
            return True
    if isinstance(r, Pair):
        if split(pair.right):
            return True
    return False

def reduce(pair):
    while explode(pair) or split(pair):
        pass
    return pair

