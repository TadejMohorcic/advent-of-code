from functools import reduce
from itertools import combinations

with open('2015/Input/2015-24.txt') as input:
    weights = []

    for line in input:
        weights.append(int(line.strip()))

def list_reduce(l):
    return reduce(lambda x, y: x*y, l)

def quantum_enlargement(n, weight_list):
    total = sum(weight_list) // n

    for i in range(len(weight_list)):
        valid = [list_reduce(x) for x in combinations(weight_list, i) if sum(x) == total]
        if valid != []:
            return min(valid)
        
part_one = quantum_enlargement(3, weights)
part_two = quantum_enlargement(4, weights)

print('--- Day 24: It Hangs in the Balance ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')