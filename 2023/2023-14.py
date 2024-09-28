import numpy as np

with open('2023/Input/2023-14.txt') as input:
    platform = []

    for line in input:
        replaced_line = line.strip().replace('.', '0').replace('O', '1').replace('#', '2')
        replaced_line = [int(n) for n in list(replaced_line)]
        platform.append(replaced_line)

    platform = np.array(platform)

def rearange_array(array_slice):
    i = 1
    while i < len(array_slice):
        j = i

        while j > 0 and array_slice[j - 1] < array_slice[j] and array_slice[j] != 2:
            current = array_slice[j]
            array_slice[j] = array_slice[j - 1]
            array_slice[j - 1] = current
            j -= 1

        i += 1

    return array_slice

def rotate_platform(platform_array, direction):
    n, m = np.shape(platform_array)

    if direction in 'NS':
        for i in range(m):
            platform_array[:, i] = rearange_array(platform_array[:, i]) if direction == 'N' else rearange_array(platform_array[:, i][::-1])[::-1]
    else:
        for i in range(n):
            platform_array[i, :] = rearange_array(platform_array[i, :]) if direction == 'W' else rearange_array(platform_array[i, :][::-1])[::-1]
    
    return platform_array

def calculate_load(platform_array, part):
    res = 0
    copy_array = np.copy(platform_array)

    if part:
        rotated = copy_array

        for i in range(160):
            rotated = rotate_platform(rotated, 'N')
            rotated = rotate_platform(rotated, 'W')
            rotated = rotate_platform(rotated, 'S')
            rotated = rotate_platform(rotated, 'E')

    else:
        rotated = rotate_platform(copy_array, 'N')

    n, m = np.shape(rotated)

    for i in range(m):
        for j in range(n):
            rock = rotated[i, j]
            res += (m - i) * rock if rock == 1 else 0

    return res

part_one = calculate_load(platform, False)
part_two = calculate_load(platform, True)

print('--- Day 14: Parabolic Reflector Dish ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')