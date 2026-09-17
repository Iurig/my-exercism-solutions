"""Module implements functions for encoding and decoding unsigned integers into VQL"""

BYTE = 8
SIZE = 2 ** (BYTE - 1)


def encode(numbers):
    encoded = []
    for n in numbers:
        curr_n = n
        n_encoded = []
        while True:
            curr_byte = curr_n % (SIZE)
            curr_n //= SIZE
            n_encoded.append(curr_byte + SIZE)
            if curr_n == 0:
                break
        n_encoded[0] -= SIZE
        n_encoded.reverse()
        encoded.extend(n_encoded)
    return encoded


def decode(bytes_):
    if bytes_[-1] >= SIZE:
        raise ValueError("incomplete sequence")
    numbers = []
    num = 0
    for byte in bytes_:
        num *= SIZE
        if byte < SIZE:
            num += byte
            numbers.append(num)
            num = 0
        else:
            num += byte - SIZE
    return numbers
