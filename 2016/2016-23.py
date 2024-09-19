with open('2016/Input/2016-23.txt') as input:
    instructions = []

    for line in input:
        instructions.append(line.strip())

def get_value(start_number):
    a = start_number
    b = a - 1

    while b > 1:
        d = a
        a = d * b
        b -= 1
    a += 73 * 91

    return a

part_one = get_value(7)
part_two = get_value(12)

print('--- Day 23: Safe Cracking ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')