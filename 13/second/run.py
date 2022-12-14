import sys
import functools

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

vv = []
vv.append([[2]])
vv.append([[6]])

for line in sys.stdin.readlines():
    line = line.strip()
    if len(line) == 0:
        continue
    vv.append(eval(line))

svv = sorted(vv, key=functools.cmp_to_key(compare), reverse=True)
i = 1
for (index, v) in enumerate(svv):
    print(index, v)
    if v == [[2]] or v == [[6]]:
        i = i * (index + 1)

print(i)

