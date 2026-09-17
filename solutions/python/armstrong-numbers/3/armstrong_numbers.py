"""Module providing a function that verifies if a number is an armstrong number."""


def is_armstrong_number(number: int) -> bool:
    """Verify if a number is an armstrong number.

    Args:
        number: number to be verified.

    Returns:
        A bool representing if it is an armstrong number.

    """
    digits = str(number)
    return number == sum(int(single_digit) ** len(digits) for single_digit in digits)
