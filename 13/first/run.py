import sys

def compare(left, right):
    if isinstance(left, int) and isinstance(right, int):
        return right - left

    if isinstance(left, int) and isinstance(right, list):
        return compare([left], right)

    if isinstance(left, list) and isinstance(right, int):
        return compare(left, [right])

    if len(left) > 0 and len(right) == 0:
        return -1

    if len(left) == 0 and len(right) > 0:
        return 1
    
    index = 0
    while index < len(left) and index < len(right):
        v = compare(left[index], right[index])
        if v != 0:
            return v
        index += 1

    if index < len(left) and index == len(right):
        return -1

    if index == len(left) and index == len(right):
        return 0

    return 1

s = 0
index = 0
while True:
    line = sys.stdin.readline()
    if len(line) == 0:
        break

    index += 1
    left = eval(line.strip())
    right = eval(sys.stdin.readline().strip())
    sys.stdin.readline()

    if compare(left, right) > 0:
        s += index

print(s)

