"""Module containing functions to determine weather a triangle is equilateral,
isosceles, or scalene, calculated in separate bool return functions.
Non triangles always return False"""


def is_triangle(sides):
    return min(sides) > 0 and 2 * max(sides) <= sum(sides)


def equilateral(sides):
    return is_triangle(sides) and max(sides) == min(sides)


def isosceles(sides):
    return is_triangle(sides) and len(set(sides)) < 3


def scalene(sides):
    return is_triangle(sides) and len(set(sides)) == 3
