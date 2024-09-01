import heapq

with open('2016/Input/2016-13.txt') as input:
    number = int(input.readline().strip())

def calculate_open(position, special_num):
    x, y = position

    if x < 0 or y < 0:
        return False
    
    num = x * (x + 3) + y * (2 * x + 1 + y) + special_num
    binary = bin(num)[2:]
    num_type = 0

    for c in str(binary):
        num_type += int(c)

    return True if num_type % 2 == 0 else False

def shortest_path(start_pos, end_pos, special_num, part):
    queue = [(0, start_pos)]
    directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]
    seen = set()

    while queue != []:
        steps, position = heapq.heappop(queue)

        if position == end_pos:
            if part:
                visited = 0
                for (s, _) in seen:
                    if s <= 50 :
                        visited += 1
                return visited
            else:
                return steps
        
        already_seen = False
        for i in range(steps):
            if (i, tuple(position)) in seen:
                already_seen = True
                continue

        if already_seen:
            continue

        seen.add((steps, tuple(position)))
        
        for d in directions:
            new_position = position.copy()
            new_position[0] += d[0]
            new_position[1] += d[1]

            if calculate_open(new_position, special_num):
                heapq.heappush(queue, (steps + 1, new_position))

part_one = shortest_path([1, 1], [31, 39], number, False)
part_two = shortest_path([1, 1], [31, 39], number, True)

print('--- Day 13: A Maze of Twisty Little Cubicles ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')