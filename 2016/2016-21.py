with open('2016/Input/2016-21.txt') as input:
    instructions = []
    for line in input:
        text = line.strip().split()
        if text[0] == 'swap':
            if text[1] == 'letter':
                instructions.append((text[0], text[2], text[-1]))
            else:
                instructions.append((text[0], int(text[2]), int(text[-1])))
        elif text[0] == 'rotate':
            if text[1] == 'based':
                instructions.append((text[0], text[-1]))
            else:
                instructions.append((text[0], text[1], int(text[2])))
        else:
            instructions.append((text[0], int(text[2]), int(text[-1])))

def swap_characters(string, index):
    index_min, index_max = min(index), max(index)
    return string[:index_min] + string[index_max] + string[index_min + 1:index_max] + string[index_min] + string[index_max + 1:]

def rotate(string, direction, steps):
    n = steps if direction == 'left' else len(string) - steps
    return string[n:] + string[:n]

def reverse(string, index):
    index_min, index_max = min(index), max(index)
    return string[:index_min] + string[index_min:index_max + 1][::-1] + string[index_max + 1:]

def move_character(string, index):
    char = string[index[0]]
    new_string = string[:index[0]] + string[index[0] + 1:]
    return new_string[:index[1]] + char + new_string[index[1]:]

def scramble_string(string, instruction_list, part):
    n = len(string)
    instruction_list = list(reversed(instruction_list)) if part else instruction_list

    for i in instruction_list:

        if i[0] == 'swap':
            try:
                index_1, index_2 = string.index(i[1]), string.index(i[2])
            except TypeError:
                index_1, index_2 = i[1], i[2]
            string = swap_characters(string, (index_1, index_2))
        elif i[0] == 'rotate':
            try:
                if part:
                    string = rotate(string, i[1], n - i[2])
                else:
                    string = rotate(string, i[1], i[2])
            except IndexError:
                index = string.index(i[1])
                if part:
                    steps = (index // 2) + 1 if index % 2 != 0 else (index + (n + 2 - index) // 2) % n
                    string = rotate(string, 'left', steps)
                else:
                    steps = 1 + index + 1 if index >= 4 else 1 + index
                    string = rotate(string, 'right', steps)
        elif i[0] == 'reverse':
            string = reverse(string, (i[1], i[2]))
        else:
            if part:
                string = move_character(string, (i[2], i[1]))
            else:
                string = move_character(string, (i[1], i[2]))

    return string

part_one = scramble_string('abcdefgh', instructions, False)
part_two = scramble_string('fbgdceah', instructions, True)

print('--- Day 21: Scrambled Letters and Hash ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')