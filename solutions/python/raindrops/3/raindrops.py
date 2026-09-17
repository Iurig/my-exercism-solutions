"""Module for converting a number to it's raindrop representation."""


def convert(number: int) -> str:
    """Convert the number to raindrop representation."""
    substitutions = {3: "Pling", 5: "Plang", 7: "Plong"}
    converted = "".join(
        item for key, item in substitutions.items() if number % key == 0
    )
    if len(converted) == 0:
        converted = str(number)
    return converted
