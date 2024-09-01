import re

with open('2016/Input/2016-15.txt') as input:
    discs = []

    i = 1
    for line in input:
        _, positions, _, start = [int(x) for x in re.findall(r'\d+', line.strip())]
        discs.append((positions, (positions - start - i) % positions))
        i += 1

def extended_euclidian_algorithm(a, b):
    r_0, r_1 = a, b
    s_0, s_1 = 1, 0
    t_0, t_1 = 0, 1

    while r_1 != 0:
        q = r_0 // r_1
        r_0, r_1 = r_1, r_0 - q * r_1
        s_0, s_1 = s_1, s_0 - q * s_1
        t_0, t_1 = t_1, t_0 - q * t_1

    return s_0, t_0

def chinese_remainder_theorem(number_list):

    while len(number_list) > 1:
        n1, a1 = number_list.pop(0)
        n2, a2 = number_list.pop(0)
        
        s, t = extended_euclidian_algorithm(n1, n2)

        x = a1 * t * n2 + a2 * s * n1

        number_list.append((n1 * n2, x % (n1 * n2)))

    _, x = number_list.pop(0)

    return x

def timing(number_list, part):
    copied_list = number_list.copy()

    if part:
        copied_list.append((11, 11 - len(number_list) - 1))
    
    return chinese_remainder_theorem(copied_list)


part_one = timing(discs, False)
part_two = timing(discs, True)

print('--- Day 15: Timing is Everything ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')