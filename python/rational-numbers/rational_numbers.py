from __future__ import division


class Rational:
    def __init__(self, number, denom):
        self.number = number
        self.denom = denom

    def __eq__(self, other):
        return self.number == other.number and self.denom == other.denom

    def __repr__(self):
        return '{}/{}'.format(self.number, self.denom)

    def __add__(self, other):
        return Rational(
            (self.number * other.denom + other.number * self.denom),
            (self.denom * other.denom)
        )

    def __sub__(self, other):
        pass

    def __mul__(self, other):
        pass

    def __truediv__(self, other):
        pass

    def __abs__(self):
        pass

    def __pow__(self, power):
        pass

    def __rpow__(self, base):
        pass
