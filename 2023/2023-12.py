from functools import cache

with open('2023/Input/2023-12.txt') as input:
    springs = []

    for line in input:
        spring, numbers = line.strip().split()
        numbers = tuple(int(n) for n in numbers.split(','))
        springs.append((spring, numbers))

@cache
def check_spring(spring, number_list):
    result = 0

    if number_list == ():
        return 1 if '#' not in spring else 0
    
    if spring == '':
        return 1 if number_list == () else 0

    if spring[0] in '.?':
        result += check_spring(spring[1:], number_list)

    if spring[0] in '#?':
        first_group = number_list[0]
        n = len(spring)
        if first_group <= n and '.' not in spring[:first_group] and (first_group == n or spring[first_group] != '#'):
            result += check_spring(spring[first_group + 1:], number_list[1:])

    return result

def valid_springs(spring_list, part):
    res = 0

    for spring, numbers in spring_list:
        if part:
            new_spring = 4 * (spring + '?') + spring
            new_numbers = 5 * numbers
            res += check_spring(new_spring, new_numbers)
        else:
            res += check_spring(spring, numbers)

    return res

part_one = valid_springs(springs, False)
part_two = valid_springs(springs, True)

print('--- Day 12: Hot Springs ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')