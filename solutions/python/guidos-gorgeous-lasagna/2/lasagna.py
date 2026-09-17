"""Functions used in preparing Guido's gorgeous lasagna.

Learn about Guido, the creator of the Python language:
https://en.wikipedia.org/wiki/Guido_van_Rossum

This is a module docstring, used to describe the functionality
of a module and its functions and/or classes.
"""

EXPECTED_BAKE_TIME = 40
PREPARATION_TIME = 2


def preparation_time_in_minutes(number_of_layers: int):
    """Calculate the preparation cooking time.

    Parameters:
        number_of_layers (int): The number of layers in the lasagna.

    Returns:
        int: The time required (in minutes) derived from 'PREPARATION_TIME'

    Function that takes the number of layers in the lasagna and returns how long it
    would take to prepare such lasagna if each layer takes 'PREPARATION_TIME' minutes
    to prepare.
    """
    return number_of_layers * PREPARATION_TIME


def bake_time_remaining(elapsed_bake_time: int):
    """Calculate the bake time remaining.

    Parameters:
        elapsed_bake_time (int): The baking time already elapsed.

    Returns:
        int: The remaining bake time (in minutes) derived from 'EXPECTED_BAKE_TIME'.

    Function that takes the actual minutes the lasagna has been in the oven as
    an argument and returns how many minutes the lasagna still needs to bake
    based on the `EXPECTED_BAKE_TIME`.
    """
    return EXPECTED_BAKE_TIME - elapsed_bake_time


def elapsed_time_in_minutes(number_of_layers: int, elapsed_bake_time: int):
    """Calculate the elapsed time preparing and cooking.

    Parameters:
        number_of_layers (int): The number of layers in the lasagna.
        elapsed_bake_time (int): The baking time already elapsed.

    Returns:
        int: The time spent preparing the lasagna and baking it.

    Function that calculates the time spent preparing the lasagna using
    'preparation_time_in_minutes()', and adds it to elapsed_bake_time to calculate the
    total time spent on the lasagna so far.
    """
    return preparation_time_in_minutes(number_of_layers) + elapsed_bake_time
