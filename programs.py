from itertools import combinations_with_replacement

l = []

for state in 'ABCDH':
    for sym in '01':
        for d in 'LR':
            l.append(sym + d + state)


for item in combinations_with_replacement(l, 8):
    print(" ".join(item))

