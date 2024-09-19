import numpy as np
import heapq

with open('2016/Input/2016-22.txt') as input:
    input.readline()
    input.readline()
    nodes = {}
    i, j = 0, 0
    for line in input:
        _, _, used, avail, _ = line.strip().split()
        used, avail = int(used[:-1]), int(avail[:-1])
        nodes[(i, j)] = [used, avail]
        if j == 30:
            i += 1
            j = 0
        else:
            j += 1

def interchangable(key, node_dict):
    valid = 0
    used = node_dict[key][0]

    for key_2 in node_dict:
        if key != key_2:
            if node_dict[key_2][1] >= used:
                valid += 1

    return valid

def valid_key(node_dict):
    valid = 0

    for key in node_dict:
        if node_dict[key][0] != 0:
            valid += interchangable(key, node_dict)

    return valid

def construct_grid(node_dict):
    node_grid = 2 * np.ones((32,31), dtype=int)

    for key in node_dict:
        if key == (31, 0):
            node_grid[key] = 5
        else:
            if node_dict[key][0] == 0:
                node_grid[key] = 0
            else:
                v = interchangable(key, node_dict)
                if v > 0:
                    node_grid[key] = 1

    return node_grid

def shortest_path(node_grid):
    queue = [(0, (28, 20))]
    directions = [(1, 0), (-1, 0), (0, -1), (0, 1)]
    seen = set()

    while queue != []:
        steps, position = heapq.heappop(queue)

        if node_grid[position] == 5:
            return steps + 5 * 30
        
        if node_grid[position] == 2 or position in seen:
            continue

        else:
            seen.add(position)
            for d in directions:
                new_position = (position[0] + d[0], position[1] + d[1])
                if 0 <= new_position[0] <= 31 and 0 <= new_position[1] <= 30:
                    heapq.heappush(queue, (steps + 1, new_position))


part_one = valid_key(nodes)
part_two = shortest_path(construct_grid(nodes))

print('--- Day 22: Grid Computing ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')