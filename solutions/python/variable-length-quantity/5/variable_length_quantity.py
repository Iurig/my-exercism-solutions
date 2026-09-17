"""Module implements functions for encoding and decoding unsigned integers into VQL"""

CONTINUATION_BIT = 0x80


def encode(numbers):
    """Encodes unsigned integer list into byte list, separating integers by a value
    of 0 on the most significant bit"""
    encoded = []
    for num in numbers:
        curr_num = num
        n_encoded = []
        while True:
            curr_byte = curr_num % (CONTINUATION_BIT)
            curr_num //= CONTINUATION_BIT
            n_encoded.append(curr_byte + CONTINUATION_BIT)
            if curr_num == 0:
                break
        n_encoded[0] -= CONTINUATION_BIT
        n_encoded.reverse()
        encoded.extend(n_encoded)
    return encoded


def decode(bytes_):
    """Decodes a byte list into integer list, encoded by separating integers by a 0
    on the most significant bit of the byte"""
    if bytes_ and bytes_[-1] >= CONTINUATION_BIT:
        raise ValueError("incomplete sequence")
    numbers = []
    num = 0
    for byte in bytes_:
        num *= CONTINUATION_BIT
        if byte < CONTINUATION_BIT:
            num += byte
            numbers.append(num)
            num = 0
        else:
            num += byte - CONTINUATION_BIT
    return numbers
