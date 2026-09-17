def is_armstrong_number(number: int) -> bool:
    digits = []
    mut_number = number
    while mut_number != 0:
        digits.append(mut_number % 10)
        mut_number //= 10
    return sum(d ** len(digits) for d in digits) == number
