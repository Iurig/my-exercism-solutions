"""Module for lackadaisical responses"""


def response(hey_bob: str) -> str:
    """Function returning lackadaisical responses."""
    if len(hey_bob.strip()) == 0:
        return "Fine. Be that way!"
    if hey_bob.strip()[-1] == "?":
        if hey_bob.isupper():
            return "Calm down, I know what I'm doing!"
        return "Sure."
    if hey_bob.isupper():
        return "Whoa, chill out!"
    return "Whatever."
