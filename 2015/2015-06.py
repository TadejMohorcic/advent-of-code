import re
import numpy as np

with open('2015/Input/2015-06.txt') as input:
    instructions = []

    for line in input:
        instruction = re.findall('([a-zA-Z ]*)\\d*.*', line.strip())[0][:-1]
        coordinates = [int(i) for i in re.findall('\\d+', line.strip())]
        instructions.append((instruction, coordinates))

def adjust_lights(grid, instructions, part):
    instruction = instructions[0]
    coordinates = instructions[1]
    start_x, start_y, end_x, end_y = coordinates[0], coordinates[1], coordinates[2], coordinates[3]

    match instruction:
        case 'toggle':
            if part:
                grid[start_x:end_x + 1, start_y:end_y + 1] += 2
            else:
                grid[start_x:end_x + 1, start_y:end_y + 1] = abs(grid[start_x:end_x + 1, start_y:end_y + 1] - 1)
        case 'turn on':
            if part:
                grid[start_x:end_x + 1, start_y:end_y + 1] += 1
            else:
                grid[start_x:end_x + 1, start_y:end_y + 1] = 1
        case 'turn off':
            if part:
                grid[start_x:end_x + 1, start_y:end_y + 1] = np.maximum(grid[start_x:end_x + 1, start_y:end_y + 1] - 1, np.zeros((end_x - start_x + 1, end_y - start_y + 1)))
            else:
                grid[start_x:end_x + 1, start_y:end_y + 1] = 0

    return grid

def update_grid(grid, instruction_list, part):
    g = np.copy(grid)
    
    for i in instruction_list:
        g = adjust_lights(g, i, part)

    return np.sum(g)

lights = np.zeros((1000, 1000), dtype=int)

part_one = update_grid(lights, instructions, False)
part_two = update_grid(lights, instructions, True)

print('--- Day 6: Probably a Fire Hazard ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part one solution: {part_two}.')