# some NatNum operations requires a copy right now (easier)
# without copying, python would us a reference
# so when n1 + n2 = n3, n1 and n3 would be the same,
# which is not what we want
from copy import copy

# NatNum type to represent set based natural numbers
class NatNum:
    # tree represents the number
    tree = frozenset()

    def __init__(self, number=0):
        assert number >= 0
        for _ in range(number):
            self.increment()

    # increment tree by `n + 1 = n ∪ {n}`
    def increment(self):
        subtree = self.tree
        self.tree = subtree | {subtree} 

    # decrement by removing the last subtree
    # TODO: maybe reimplement this in the real
    #       set operation way. e.g. ⋃n instead
    #       of removing the subtree
    def decrement(self):
        assert self.to_int() > 0
        tree = self.tree
        self.tree = frozenset(list(tree)[:-1])

    # return the number the tree represents
    # this function doesn't check the actual correctness
    # of the tree, which doesn't matter when
    # the tree is not modified by hand
    def to_int(self):
        return len(self.tree)

    def __eq__(self, other):
        return self.to_int() == other.to_int()

    # adds two NatNums by incrementing itself `n times`
    # where `n` is the represented number from `other`
    def __add__(self, other):
        new_num = copy(self)
        for _ in range(other.to_int()):
            new_num.increment()
        return new_num

    # adds two NatNums by decrementing itself `n times`
    # where `n` is the represented number from `other`
    # panics if `other > self` because this would result
    # in a negative number
    def __sub__(self, other):
        assert self.to_int() >= other.to_int()
        new_num = copy(self)
        for _ in range(other.to_int()):
            new_num.decrement()
        return new_num

    # multiply two numbers by
    # creating a new number and incrementing it `n` times
    # where `n = self * other`
    def __mul__(self, other):
        if isinstance(other, int):
            new_num = NatNum(self.to_int() * other)
        else:
            new_num = NatNum(self.to_int() * other.to_int())
            return new_num

    # divide two numbers by
    # creating a new number and incrementing it `n` times
    # where `n = self / other`
    # panics if `self / other` results in a float
    # Note: truediv shouldn't be used for natural numbers
    #       but writing `//` for division looks ugly
    #       so both `truediv` and `floordiv` are implemented
    def __truediv__(self, other):
        steps = self.to_int() // other.to_int()
        assert isinstance(steps, int) == True

        new_num = NatNum(steps)
        return new_num

    # divide two numbers by
    # creating a new number and incrementing it `n` times
    # where `n = self / other`
    # panics if `self / other` results in a float
    def __floordiv__(self, other):
        steps = self.to_int() // other.to_int()
        assert isinstance(steps, int) == True

        new_num = NatNum(steps)
        return new_num

    # power one number by another
    def __pow__(self, other):
        if isinstance(other, self.__class__):
            # n^0 = 1
            if other.to_int() == 0:
                new_num = NatNum(1)
                return new_num
            # n^m
            else:
                new_num = copy(self)
                for _ in range(1, other.to_int()):
                    new_num = new_num * self
                
                return new_num

        elif isinstance(other, int):
            # n^0 = 1
            if other == 0:
                new_num = NatNum(1)
                return new_num
            # n^m
            else:
                new_num = copy(self)
                for _ in range(1, other):
                    new_num = new_num * self
                
                return new_num

        else:
            exit("unsuported type")
        



def tests():
    print("running examples")
    # create numbers
    # default (no value) => 0
    num1 = NatNum()
    num2 = NatNum(2)
    num3 = NatNum(5)

    assert num1.to_int() == 0
    assert num2.to_int() == 2
    assert num3.to_int() == 5

    # increment number
    num2.increment()
    assert num2.to_int() == 3

    # decrement number
    num2.decrement()
    assert num2.to_int() == 2

    # using both works (one doesn't break the other)
    num2.decrement()
    num2.decrement()
    assert num2.to_int() == 0
    assert num2 == num1
    num2.increment()
    num2.increment()
    assert num2.to_int() == 2

    # add numbers
    tmp = num1 + num2
    assert tmp.to_int() == 2
    tmp = num2 + num3
    assert tmp.to_int() == 7

    # subtract numbers
    tmp = num2 - num1
    assert tmp.to_int() == 2
    tmp = num3 - num2
    assert tmp.to_int() == 3

    # multiply numbers
    tmp = num2 * num1
    assert tmp.to_int() == 0
    tmp = num2 * num3
    assert tmp.to_int() == 10

    # divide numbers
    num4 = NatNum(10)
    tmp = num4 / num2
    assert tmp.to_int() == 5

    # exponentials
    tmp = num2 ** num1
    assert tmp.to_int() == 1
    tmp = num2 ** 2
    assert tmp.to_int() == 4
    tmp = num2 ** num3
    assert tmp.to_int() == 32

    two = NatNum(2)
    three = NatNum(3)

    # 2^(2^3) = 2^(8) = 256
    tmp1 = two ** (two ** three)
    print(tmp1.to_int())

    # (2^2)^3 = 2^(6) = 64
    tmp2 = (two ** two) ** three
    print(tmp2.to_int())

    # 2^(2*3) = 2^(6) = 64
    tmp3 = two ** (two * three)
    print(tmp3.to_int())

    assert tmp1 != tmp2
    assert tmp1 != tmp3
    assert tmp2 == tmp3

    #if all asserts succeed, print "success"
    print("success")



if __name__ == "__main__":
    tests()