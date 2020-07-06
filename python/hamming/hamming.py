def distance(strand_a, strand_b):
    if len(strand_a) != len(strand_b):
        raise ValueError("%d != %d", len(strand_a), len(strand_b))

    value = 0
    for idx, a_value in enumerate(strand_a):
        if a_value != strand_b[idx]:
            value += 1
    return value
