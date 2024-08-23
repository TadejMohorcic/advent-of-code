import numpy as np

with open('2016/Input/2016-08.txt') as input:
    instructions = []
    
    for line in input:
        text = line.strip().split()
        if len(text) == 2:
            dimensions = [int(x) for x in text[1].split('x')]
            instructions.append(dimensions)
        else:
            direction = text[1]
            index = int(text[2].split('=')[-1])
            size = int(text[-1])
            instructions.append([direction, index, size])

def create_rectangle(screen, size):
    for i in range(size[0]):
        for j in range(size[1]):
            screen[j, i] = 1

    return screen

def shift_row(screen, row_id, shift):
    row_len = np.shape(screen)[1]
    row = screen[row_id]

    new_row = np.concatenate((row[row_len - shift:], row[:row_len - shift]))
    screen[row_id] = new_row

    return screen

def shift_column(screen, column_id, shift):
    column_len = np.shape(screen)[0]
    column = screen[:, column_id]

    new_column = np.concatenate((column[column_len - shift:], column[:column_len - shift]))
    screen[:, column_id] = new_column

    return screen

def update_screen(instruction_list, size):
    screen = np.zeros(size, dtype=int)

    for instruction in instruction_list:
        if len(instruction) == 2:
            screen = create_rectangle(screen, instruction)
        else:
            if instruction[0] == 'row':
                screen = shift_row(screen, instruction[1], instruction[2])
            else:
                screen = shift_column(screen, instruction[1], instruction[2])

    return screen

def display_screen(screen):
    y, x = np.shape(screen)

    for j in range(y):
        line = ''
        for i in range(x):
            if screen[j, i] == 0:
                line += ' '
            else:
                line += '#'
        print(line)

updated_screen = update_screen(instructions, (6, 50))
part_one = np.sum(updated_screen)

print('--- Day 8: Two-Factor Authentication ---')
print(f' -  Part one solution: {part_one}.')
display_screen(updated_screen)