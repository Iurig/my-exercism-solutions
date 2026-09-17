"""This module can determine the number of grains of wheat on a certain square of a
chess board if the first square has 1 grain, and every subsequent square has double
the amount of grains of the previous"""


def square(number):
    if number < 1 or number > 64:
        raise ValueError("square must be between 1 and 64")
    return 2 ** (number - 1)


def total():
    squares = 64
    return 2**squares - 1
