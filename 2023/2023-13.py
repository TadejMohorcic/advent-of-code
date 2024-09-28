import numpy as np

with open('2023/Input/2023-13.txt') as input:
    islands = []
    current_island = []

    for line in input:
        if line == '\n':
            islands.append(np.array(current_island))
            current_island = []
        else:
            current_island.append(list(line.strip()))

    if current_island != []:
        islands.append(np.array(current_island))

def is_reflection(first_part, second_part, direction):
    if direction == 'NS':
        return True if (first_part == np.flipud(second_part)).all() else False
    
    if direction == 'EW':
        return True if (first_part == np.fliplr(second_part)).all() else False

def check_mirror(mirror_array):
    reflections = []
    n, m = np.shape(mirror_array)

    for i in range(1, n):
        k = min(i, n - i)
        if (is_reflection(mirror_array[(i - k):i, :], mirror_array[i:(i + k), :], 'NS')):
            reflections.append(100 * i)

    for i in range(1, m):
        k = min(i, m - i)
        if (is_reflection(mirror_array[:, (i - k):i], mirror_array[:, i:(i + k)], 'EW')):
            reflections.append(i)

    return reflections

def repair_mirror(mirror_array, i, j):
    copied_mirror = np.copy(mirror_array)

    if mirror_array[i, j] == '#':
        copied_mirror[i, j] = '.'
    else:
        copied_mirror[i, j] = '#'

    return copied_mirror

def reflections(island_list, part):
    res = 0

    for mirror in island_list:
        if part:
            original = check_mirror(mirror)[0]
            n, m = np.shape(mirror)
            found = False

            for i in range(n):
                for j in range(m):
                    repaired_mirror = repair_mirror(mirror, i, j)
                    fixed_reflection = check_mirror(repaired_mirror)

                    for reflection in fixed_reflection:
                        if reflection != original:
                            res += reflection
                            found = True
                    
                    if found:
                        break
                else:
                    continue

                break
            
        else:
            res += check_mirror(mirror)[0]

    return res

part_one = reflections(islands, False)
part_two = reflections(islands, True)

print('--- Day 13: Point of Incidence ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part one solution: {part_two}.')