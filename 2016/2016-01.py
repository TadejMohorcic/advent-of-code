with open('2016\\Input\\2016-01.txt') as input:
    directions = input.readline().strip().split(', ')

def follow_directions(direction_list, part):
    x, y = 0, 0
    way = 0

    directions = [(1, 0), (0, -1), (-1, 0), (0, 1)]

    visited = set()
    visited.add((x, y))

    for d in direction_list:
        dx, dy = directions[way]
        distance = int(d[1:])

        if d[0] == 'R':
            direction = 1
            way = (way + 1) % 4
        else:
            direction = -1
            way = (way - 1) % 4

        for i in range(1, distance + 1):
            test_x = x + i * direction * dx
            test_y = y + i * direction * dy

            if (test_x, test_y) in visited and part:
                return test_x, test_y
            
            visited.add((test_x, test_y))

        x += distance * direction * dx
        y += distance * direction * dy
    
    return x, y

def calculate_distance(x, y):
    return abs(x) + abs(y)

x1, y1 = follow_directions(directions, False)
part_one = calculate_distance(x1, y1)

x2, y2 = follow_directions(directions, True)
part_two = calculate_distance(x2, y2)

print('--- Day 1: No Time for a Taxicab ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')