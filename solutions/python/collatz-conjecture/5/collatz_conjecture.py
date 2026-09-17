"""Module providing Collatz Conjecture-related similation tooling."""


def collatz(number: int) -> int:
    """The Collatz function itself.

    Args:
        number: The number to be operated.

    Returns:
        The next number given by the Collatz recursion
    """

    if number % 2 == 0:
        return number // 2
    return 3 * number + 1


def steps(iterable_number: int) -> int:
    """Calculate the number of steps of the Collatz Conjecture function until 1 is
    reached.

    Args:
        number: The starting number.

    Returns:
        The number of steps starting at number until 1 is reached.

    """

    if iterable_number < 1:
        raise ValueError("Only positive integers are allowed")
    steps = 0
    while iterable_number != 1:
        iterable_number = collatz(iterable_number)
        steps += 1
    return steps
