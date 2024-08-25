with open('2016/Input/2016-12.txt') as input:
    instructions = []
    value_lists = list('abcd')

    for line in input:
        text = line.strip().split()
        if len(text) == 2:
            instructions.append(text)
        else:
            ins, val_1, val_2 = text[0], text[1], text[2]
            if text[1] not in value_lists:
                val_1 = int(val_1)
            if text[2] not in value_lists:
                val_2 = int(val_2)
            instructions.append([ins, val_1, val_2])

def get_starting_values(instruction_list, value_list):
    i = 0
    while i != 10:
        instruction = instruction_list[i]
        match instruction[0]:
            case 'cpy':
                val_1, val_2 = instruction[1], instruction[2]
                value_list[val_2] = value_list[val_1] if val_1 in value_list else val_1
                i += 1
            case 'inc':
                value_list[instruction[1]] += 1
                i += 1
            case 'dec':
                value_list[instruction[1]] -= 1
                i += 1
            case 'jnz':
                val_1, val_2 = instruction[1], instruction[2]
                if val_1 in value_list:
                    i += val_2 if value_list[val_1] != 0 else 1
                else:
                    i += val_2 if val_1 != 0 else 1

    return list(value_list.values())

def alternative_fibonacci(value_list):
    a, b, c, d = value_list
    while d != 0:
        c = a
        a += b
        b = c
        d -= 1

    return a + 19 * 11

def get_value(instruction_list, part):
    register = {'a': 0, 'b': 0, 'c': 0, 'd': 0}
    register['c'] += 1 if part else 0
    value = alternative_fibonacci(get_starting_values(instruction_list, register))
    return value

part_one = get_value(instructions, False)
part_two = get_value(instructions, True)

print('--- Day 12: Leonardos Monorail ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')