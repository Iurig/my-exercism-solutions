"""
This exercise stub and the test suite contain several enumerated constants.

Enumerated constants can be done with a NAME assigned to an arbitrary,
but unique value. An integer is traditionally used because it’s memory
efficient.
It is a common practice to export both constants and functions that work with
those constants (ex. the constants in the os, subprocess and re modules).

You can learn more here: https://en.wikipedia.org/wiki/Enumerated_type
"""

# Possible sublist categories.
# Change the values as you see fit.
SUBLIST = 0
SUPERLIST = 1
EQUAL = 2
UNEQUAL = 3


def _is_sublist_eq(small_list, big_list):
    if not small_list:
        return True
    for starting_index in range(len(big_list) - len(small_list) + 1):
        sublist_at_starting_index = True
        for index_offset, element in enumerate(small_list):
            if big_list[starting_index + index_offset] != element:
                sublist_at_starting_index = False
                break
        if sublist_at_starting_index:
            return True
    return False


def sublist(list_one, list_two):
    match (_is_sublist_eq(list_one, list_two), _is_sublist_eq(list_two, list_one)):
        case (True, True):
            return EQUAL
        case (True, False):
            return SUBLIST
        case (False, True):
            return SUPERLIST
        case (False, False):
            return UNEQUAL
