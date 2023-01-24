from sys import argv

def max(num1, num2):
    if num1 > num2: return num1
    else: return num2

print(max(argv[1], argv[2]))
