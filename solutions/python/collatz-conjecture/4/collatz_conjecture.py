"""Module providing Collatz Conjecture-related similation tooling."""


def collatz(n: int) -> int:
    """The Collatz function itself.

    Args:
        number: The number to be operated.

    Returns:
        The next number given by the Collatz recursion
    """

    if n % 2 == 0:
        return n // 2
    return 3 * n + 1


def steps(number: int) -> int:
    """Calculate the number of steps of the Collatz Conjecture function until 1 is
    reached.

    Args:
        number: The starting number.

    Returns:
        The number of steps starting at number until 1 is reached.

    """

    if number < 1:
        raise ValueError("Only positive integers are allowed")
    steps = 0
    while number != 1:
        number = collatz(number)
        steps += 1
    return steps
