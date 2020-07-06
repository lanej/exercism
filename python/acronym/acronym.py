import re


def abbreviate(words):
    abbreviation = ""
    for word in re.split('[\s-]', words):
        letter = re.search('[a-zA-Z]', word)
        if letter != None:
            abbreviation += letter.group(0).upper()
    return abbreviation
