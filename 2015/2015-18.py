import numpy as np

size = 100

with open('2015/Input/2015-18.txt') as input:
    grid = np.zeros((100, 100), dtype=int)

    i = 0
    for line in input:
        text = line.strip()
        j = 0
        for char in text:
            if char == '#':
                grid[i][j] = 1
            j += 1
        i += 1

def count_neighbours(grid, index):
    n = 0
    i, j = index

    for k in range(-1,2):
        if i + k < 0 or i + k >= size:
            continue
        else:
            for l in range(-1,2):
                if j + l < 0 or j + l >= size or k == 0 and l == 0:
                    continue
                else:
                    n += grid[i + k, j + l]

    return n

def update_grid(grid, time, part):

    for i in range(time):
        h = np.zeros((size, size), dtype=int)

        for i in range(size):
            for j in range(size):
                if part:
                    if (i, j) in [(0, 0), (0, 99), (99, 0), (99, 99)]:
                        h[i, j] = 1

                n = count_neighbours(grid, (i, j))

                if n == 3 or n == 2 and grid[i, j] == 1:
                    h[i, j] = 1

        grid = h

    return np.sum(grid)

grid_2 = np.copy(grid)
grid_2[0, 0] = 1
grid_2[0, 99] = 1
grid_2[99, 0] = 1
grid_2[99, 99] = 1

part_one = update_grid(grid, 100, False)
part_two = update_grid(grid_2, 100, True)

print('--- Day 18: Like a GIF For Your Yard ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')