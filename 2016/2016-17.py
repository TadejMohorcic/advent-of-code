import heapq
import hashlib

open_doors = list('bcdef')

with open('2016/Input/2016-17.txt') as input:
    text = input.readline().strip()

def shortest_path(start_pos, end_pos, string, part):
    queue = [(0, start_pos, string)]
    directions = [(0, -1), (0, 1), (-1, 0), (1, 0)]
    directions_letters = list('UDLR')
    n = len(string)
    longest_path = 0

    while queue != []:
        steps, position, s = heapq.heappop(queue)

        if position == end_pos:
            if part:
                longest_path = max(longest_path, steps)
                continue
            else:
                return s[n:]
        
        encode = hashlib.md5((s).encode()).hexdigest()[:4]

        for i in range(4):
            if encode[i] in open_doors:
                new_s = s + directions_letters[i]
                new_pos = (position[0] + directions[i][0], position[1] + directions[i][1])
                if 0 <= new_pos[0] < 4 and 0 <= new_pos[1] < 4:
                    heapq.heappush(queue, (steps + 1, new_pos, new_s))

    return longest_path
        
part_one = shortest_path((0, 0), (3, 3), text, False)
part_two = shortest_path((0, 0), (3, 3), text, True)

print('--- Day 17: Two Steps Forward ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')