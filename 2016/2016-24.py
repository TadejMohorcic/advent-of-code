import heapq
import time

with open('2016/Input/2016-24.txt') as input:
    grid = []
    
    for line in input:
        grid.append(line.strip())

def find_number(grid_map, char):
    for i in range(len(grid_map)):
        for j in range(len(grid_map[0])):
            if grid_map[i][j] == char:
                return (i, j)
            
def path_length(grid_map, start_char, end_char):
    start = find_number(grid_map, start_char)
    seen = set()

    queue = [(0, start)]
    directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]

    while queue != []:
        steps, position = heapq.heappop(queue)
        row, col = position

        if grid_map[row][col] == end_char:
            return steps
        
        if (row, col) in seen:
            continue
        
        for d in directions:
            seen.add((row, col))
            new_row, new_col = row + d[0], col + d[1]
            if grid_map[new_row][new_col] != '#':
                heapq.heappush(queue, (steps + 1, (new_row, new_col)))

def create_graph(grid_map):
    graph = {}
    nums = '1234567'

    for i in range(len(nums)):
        char_1 = nums[i]
        for j in range(i, len(nums)):
            char_2 = nums[j]
            if char_1 != char_2:
                length = path_length(grid_map, char_1, char_2)

                if char_1 not in graph:
                    graph[char_1] = {}

                graph[char_1][char_2] = length

                if char_2 not in graph:
                    graph[char_2] = {}

                graph[char_2][char_1] = length

    return graph

def shortest_path(grid_map, graph, part):
    queue = []
    min_cost = 1e5
    nums = '1234567'

    for c in nums:
        queue.append((path_length(grid_map, '0', c), [c]))

    while queue != []:
        cost, path = heapq.heappop(queue)

        for char in nums:
            if char not in path:
                new_cost = cost + graph[path[-1]][char]
                new_path = path.copy()
                new_path.append(char)
                if len(new_path) == 7:
                    if part:
                        updated_cost = new_cost + path_length(grid_map, '0', char)
                        min_cost = updated_cost if updated_cost < min_cost else min_cost
                    else:
                        min_cost = new_cost if new_cost < min_cost else min_cost
                else:
                    heapq.heappush(queue, (new_cost, new_path))

    return min_cost

part_one = shortest_path(grid, create_graph(grid), False)
part_two = shortest_path(grid, create_graph(grid), True)

print('--- Day 24: Air Duct Spelunking ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')