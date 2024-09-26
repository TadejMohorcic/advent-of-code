from math import gcd

with open('2023/Input/2023-08.txt') as input:
    directions = input.readline().strip()
    states = {}

    for line in input:
        if line != '\n':
            start, left_right = line.strip().split(' = ')
            left, right = left_right.split(', ')
            states[start] = (left[1:], right[:-1])

def navigate(state_dict, start, end, direction_string):
    steps = 0
    n = len(directions)
    current_state = start

    while True:
        direction = direction_string[steps % n]

        current_state = state_dict[current_state][0] if direction == 'L' else state_dict[current_state][1]
        steps += 1

        if current_state in end:
            return steps
        
def find_start_end(state_dict, char):
    start_end_list = []

    for key in state_dict:
        if key[-1] == char:
            start_end_list.append(key)

    return start_end_list

def find_cycles(state_dict, direction_string):
    start_list = find_start_end(state_dict, 'A')
    end_list = find_start_end(state_dict, 'Z')
    cycles = []

    for start in start_list:
        cycles.append(navigate(state_dict, start, end_list, direction_string))

    res = 1

    for c in cycles:
        res = res * c // gcd(res, c)

    return res

part_one = navigate(states, 'AAA', 'ZZZ', directions)
part_two = find_cycles(states, directions)

print('--- Day 8: Haunted Wasteland ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')