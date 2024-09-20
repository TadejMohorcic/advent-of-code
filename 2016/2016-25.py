with open('2016/Input/2016-25.txt') as input:
    for line in input:
        line.strip()

def smallest_number():
    for i in range(200):
        b = bin(i + 365 * 7)[::-1][:-2]
        if b == '010101010101':
            return i

part_one = smallest_number()

print('--- Day 25: Clock Signal ---')
print(f' -  Part one solution: {part_one}.')