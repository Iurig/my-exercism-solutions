"""Module containing functions to determine weather a triangle is equilateral,
isosceles, or scalene, calculated in separate bool return functions.
Non triangles always return False"""


def is_triangle(sides):
    sides.sort()
    return sides[0] > 0 and sides[2] <= sides[0] + sides[1]


def equilateral(sides):
    return is_triangle(sides) and sides[0] == sides[1] == sides[2]


def isosceles(sides):
    return is_triangle(sides) and (
        sides[0] == sides[1] or sides[0] == sides[2] or sides[1] == sides[2]
    )


def scalene(sides):
    return is_triangle(sides) and not isosceles(sides)
