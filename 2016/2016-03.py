with open('2016\\Input\\2016-03.txt') as input:
    triangles = []
    for line in input:
        triangles.append([int(x) for x in line.strip().split()])

def check_triangle(x, y, z):
    if x + y > z and x + z > y and y + z > x:
        return 1
    else:
        return 0

def valid_triangles(triangle_list, part):
    count = 0

    multiplier = 1 if part else 0
    comultiplier = 1 - multiplier

    for i in range(0, len(triangle_list), 3):
        for j in range(3):
            l1 = triangle_list[i + comultiplier * j + multiplier * 0][comultiplier * 0 + multiplier * j]
            l2 = triangle_list[i + comultiplier * j + multiplier * 1][comultiplier * 1 + multiplier * j]
            l3 = triangle_list[i + comultiplier * j + multiplier * 2][comultiplier * 2 + multiplier * j]
            count += check_triangle(l1, l2, l3)

    return count

part_one = valid_triangles(triangles, False)
part_two = valid_triangles(triangles, True)

print('--- Day 3: Squares With Three Sides ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')