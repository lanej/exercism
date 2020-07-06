from collections import defaultdict
import re


def count_words(sentence):
    count = defaultdict(lambda: 0)
    for word in re.split('\W+', sentence):
        if len(word) == 0:
            continue
        count[word.lower()] += 1

    print(count)
    return count
