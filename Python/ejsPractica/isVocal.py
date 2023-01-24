from sys import argv
from re import match

def isVocal(char):
    return len(char) == 1 and ("[aeiou]", char) != None

print(isVocal(argv[1]))
