import heapq

with open('2023/Input/2023-17.txt') as input:
    city = []

    for line in input:
        city.append([int(x) for x in list(line.strip())])

def calculate_heat_loss(city_map, rows, cols):
    old_row, new_row = rows
    old_col, new_col = cols
    res = 0
    
    if old_row == new_row:
        if old_col == min(old_col, new_col):
            for i in range(1, new_col - old_col + 1):
                res += city_map[old_row][old_col + i]
        else:
            for i in range(old_col - new_col):
                res += city_map[old_row][new_col + i]
    else:
        if old_row == min(old_row, new_row):
            for i in range(1, new_row - old_row + 1):
                res += city_map[old_row + i][old_col]
        else:
            for i in range(old_row - new_row):
                res += city_map[new_row + i][old_col]

    return res

def minimal_loss(city_map, min_steps, max_steps):
    n, m = len(city_map), len(city_map[0])
    directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]
    queue = [(0, 0, 0, 0, 0, 0)]
    visited = set()

    while queue != []:
        heat_loss, row, col, drow, dcol, distance = heapq.heappop(queue)

        if row == n - 1 and col == m - 1:
            return heat_loss

        if (row, col, drow, dcol, distance) in visited:
            continue
        else:
            visited.add((row, col, drow, dcol, distance))

        if distance < max_steps and (drow, dcol) != (0, 0):
            new_row = row + drow
            new_col = col + dcol
            if 0 <= new_row < n and 0 <= new_col < m:
                heapq.heappush(queue, (heat_loss + city_map[new_row][new_col], new_row, new_col, drow, dcol, distance + 1))

        for d in directions:
            if d != (drow, dcol) and d != (-drow, -dcol):
                new_row = row + d[0] * min_steps
                new_col = col + d[1] * min_steps
                if 0 <= new_row < n and 0 <= new_col < m:
                    new_heat_loss = calculate_heat_loss(city_map, (row, new_row), (col, new_col))
                    heapq.heappush(queue, (heat_loss + new_heat_loss, new_row, new_col, d[0], d[1], min_steps))

part_one = minimal_loss(city, 1, 3)
part_two = minimal_loss(city, 4, 10)

print('--- Day 17: Clumsy Crucible ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')