"""Module that calculates weather a given year is a leap year or not"""


def leap_year(year):
    return (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0)
