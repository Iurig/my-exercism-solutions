memoise_combinations = {}


def combinations(target, size, exclude):
    if (target, size, tuple(sorted(exclude))) in memoise_combinations:
        return memoise_combinations[(target, size, tuple(sorted(exclude)))]

    if size == 1:
        if 1 <= target <= 9 and target not in exclude:
            return [[target]]
        return []
    if size < 1:
        raise ValueError("size must be a positive integer")

    comb_set = set()
    for value in range(1, min(10, target - ((size - 1) * size) // 2)):
        if value not in exclude:
            prev = combinations(target - value, size - 1, sorted(exclude + [value]))
            print(target - value, size - 1, exclude + [value], prev)
            for comb in prev:
                comb_set.add(tuple(sorted([value] + comb)))
    memoise_combinations[(target, size, tuple(sorted(exclude)))] = sorted(
        [list(comb) for comb in comb_set]
    )

    return memoise_combinations[(target, size, tuple(sorted(exclude)))]
