with open('2023/Input/2023-16.txt') as input:
    cave = []

    for line in input:
        cave.append(line.strip())

def beam_path(layout, start_index, start_direction):
    row, col = start_index
    drow, dcol = start_direction
    n, m = len(layout), len(layout[0])

    beam = set()
    queue = [(row, col, drow, dcol)]

    while queue != []:
        y, x, dy, dx = queue.pop()

        if ((y, x, dy, dx)) in beam:
            continue
        else:
            beam.add((y, x, dy, dx))

        current = layout[y][x]

        match current:
            case '.':
                if 0 <= y + dy < n and 0 <= x + dx < m:
                    queue.append((y + dy, x + dx, dy, dx))
            case '-':
                if dy == 0:
                    if 0 <= x + dx < m:
                        queue.append((y, x + dx, dy, dx))
                else:
                    if 0 <= x - 1:
                        queue.append((y, x - 1, 0, -1))
                    if x + 1 < m:
                        queue.append((y, x + 1, 0, 1))
            case '|':
                if dx == 0:
                    if 0 <= y + dy < n:
                        queue.append((y + dy, x, dy, dx))
                else:
                    if 0 <= y - 1:
                        queue.append((y - 1, x, -1, 0))
                    if y + 1 < n:
                        queue.append((y + 1, x, 1, 0))
            case '\\':
                if 0 <= y + dx < n and 0 <= x + dy < m:
                    queue.append((y + dx, x + dy, dx, dy))
            case '/':
                if 0 <= y - dx < n and 0 <= x - dy < m:
                    queue.append((y - dx, x - dy, -dx, -dy))

    return len({(y, x) for (y, x, _, _) in beam})

def best_configuration(layout):
    res = 0
    n, m = len(layout), len(layout[0])

    for i in range(n):
        energized_left = beam_path(layout, (i, 0), (0, 1))
        energized_right = beam_path(layout, (i, m - 1), (0, -1))
        res = max(res, energized_left, energized_right)

    for i in range(m):
        energized_top = beam_path(layout, (0, i), (1, 0))
        energized_bottom = beam_path(layout, (n - 1, i), (-1, 0))
        res = max(res, energized_top, energized_bottom)

    return res

part_one = beam_path(cave, (0, 0), (0, 1))
part_two = best_configuration(cave)

print('--- Day 16: The Floor Will Be Lava ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')