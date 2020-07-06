def is_isogram(string):
    letters = set()
    for c in string:
        if not c.isalpha():
            continue

        nc = c.lower()
        if nc in letters:
            return False
        letters.add(nc)

    return True
