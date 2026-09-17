"""Module implements functions for encoding and decoding unsigned integers into VQL"""

BYTE = 8
SIZE = 2 ** (BYTE - 1)


def encode(numbers):
    """Encodes unsigned integer list into byte list, separating integers by a value
    of 0 on the most significant bit"""
    encoded = []
    for num in numbers:
        curr_num = num
        n_encoded = []
        while True:
            curr_byte = curr_num % (SIZE)
            curr_num //= SIZE
            n_encoded.append(curr_byte + SIZE)
            if curr_num == 0:
                break
        n_encoded[0] -= SIZE
        n_encoded.reverse()
        encoded.extend(n_encoded)
    return encoded


def decode(bytes_):
    """Decodes a byte list into integer list, encoded by separating integers by a 0
    on the most significant bit of the byte"""
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
