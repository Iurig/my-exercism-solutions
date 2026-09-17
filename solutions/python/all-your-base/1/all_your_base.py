"""Module for converting between two numerical bases, representing bases and digits
as python's int type"""


def rebase(input_base, digits, output_base):
    if input_base < 2:
        raise ValueError("input base must be >= 2")
    if output_base < 2:
        raise ValueError("output base must be >= 2")
    power = 1
    value = 0
    for d in reversed(digits):
        if d < 0 or d >= input_base:
            raise ValueError("all digits must satisfy 0 <= d < input base")
        value += d * power
        power *= input_base

    output_digits = list()
    while value != 0:
        output_digits.append(value % output_base)
        value //= output_base

    output_digits.reverse()

    if output_digits == []:
        return [0]

    return output_digits
