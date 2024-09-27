with open('2023/Input/2023-11.txt') as input:
    space = []

    for line in input:
        space.append(line.strip())

def empty_rows_and_columns(space_map):
    n1, n2 = len(space_map), len(space_map[0])
    rows = []
    columns = []

    i = 0
    for _ in space_map:
        if space_map[i] == n2 * '.':
            rows.append(i)
        i += 1

    for i in range(n2):
        empty_col = True

        for j in range(n1):
            if space_map[j][i] == '#':
                empty_col = False

        if empty_col:
            columns.append(i)

    return rows, columns

def find_galaxies(space_map):
    galaxies = []
    i = 0

    for _ in space_map:
        j = 0

        for char in space_map[i]:
            if char == '#':
                galaxies.append((i, j))
            j += 1

        i += 1

    return galaxies

def calculate_distance(space_map, factor):
    galaxies = find_galaxies(space_map)
    rows, columns = empty_rows_and_columns(space_map)
    res = 0

    for i in range(len(galaxies)):
        for j in range(i + 1, len(galaxies)):
            galaxy_1, galaxy_2 = galaxies[i], galaxies[j]
            expand = 0

            for r in rows:
                if min(galaxy_1[0], galaxy_2[0]) < r < max(galaxy_1[0], galaxy_2[0]):
                    expand += 1
            
            for c in columns:
                if min(galaxy_1[1], galaxy_2[1]) < c < max(galaxy_1[1], galaxy_2[1]):
                    expand += 1

            res += abs(galaxy_1[0] - galaxy_2[0]) + abs(galaxy_1[1] - galaxy_2[1]) + expand * (factor - 1)

    return res

part_one = calculate_distance(space, 2)
part_two = calculate_distance(space, 1000000)

print('--- Day 11: Cosmic Expansion ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')