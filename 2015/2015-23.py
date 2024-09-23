with open('2015/Input/2015-23.txt') as input:
    instructions = []

    for line in input:
        txt = line.strip().split()
        if len(txt) == 3:
            num = txt[2]
            number = int(num[1:]) if num[0] == '+' else -int(num[1:])
            instructions.append((txt[0], txt[1][:-1], number))
        else:
            try:
                num = txt[1]
                number = int(num[1:]) if num[0] == '+' else -int(num[1:])
                instructions.append((txt[0], number))
            except ValueError:
                instructions.append((txt[0], txt[1]))


def starting_number(value, i, instruction_list):
    instruction = instruction_list[i]

    match instruction[0]:
        case 'inc':
            return starting_number(value + 1, i + 1, instruction_list)
        case 'tpl':
            return starting_number(3 * value, i + 1, instruction_list)
        case _:
            return value


def collatz(num, steps):
    if num == 1:
        return steps
    else:
        return collatz(3 * num + 1, steps + 1) if num % 2 != 0 else collatz(num // 2, steps + 1)

a = starting_number(0, 1, instructions)
part_one = collatz(a, 0)

b = starting_number(1, 19, instructions)
part_two = collatz(b, 0)

print('--- Day 23: Opening the Turing Lock ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')