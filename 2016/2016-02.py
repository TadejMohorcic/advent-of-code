with open('2016\\Input\\2016-02.txt') as input:
    instructions = []
    for line in input:
        instructions.append(line.strip())

def get_password(instruction_list, numpad, start, part):
    password = ''
    y, x = start[0], start[1]

    multiplier = 1 if part else 0

    for instruction in instruction_list:
        for c in instruction:
            match c:
                case 'U':
                    y = max(multiplier * abs(x - 2), y - 1)
                case 'D':
                    y = min(multiplier * (-abs(x - 2) + 2) + 2, y + 1)
                case 'R':
                    x = min(multiplier * (-abs(y - 2) + 2) + 2, x + 1)
                case 'L':
                    x = max(multiplier * abs(y - 2), x - 1)

        password += numpad[y][x]

    return password

numpad = [['1','2','3'],['4','5','6'],['7','8','9']]
start = 1, 1
part_one = get_password(instructions, numpad, start, False)

numpad2 = [['0','0','1','0','0'],['0','2','3','4','0'],['5','6','7','8','9'],['0','A','B','C','0'],['0','0','D','0','0']]
start2 = 2, 0
part_two = get_password(instructions, numpad2, start2, True)

print('--- Day 2: Bathroom Security ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')