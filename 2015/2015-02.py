import re

with open('2015/Input/2015-02.txt') as input:
    dimensions_list = []
    
    for line in input:
        dimensions = [int(i) for i in re.findall('\\d+', line.strip())]
        dimensions_list.append(dimensions)

def calculate_wrapping_paper(dimensions, part):
    a, b, c = dimensions[0], dimensions[1], dimensions[2]    

    if part:
        sorted_list = sorted([a, b, c])
        x, y = sorted_list[0], sorted_list[1]
        return a * b * c + 2 * (x + y)
    else:
        front, side, top = a * b, a * c, b * c
        slack = min(front, side, top)

        return 2 * front + 2 * side + 2 * top + slack
    
def wrapping_paper(dimension_list, part):
    count = 0
    for d in dimension_list:
        count += calculate_wrapping_paper(d, part)

    return count

part_one = wrapping_paper(dimensions_list, False)
part_two = wrapping_paper(dimensions_list, True)

print('--- Day 2: I Was Told There Would Be No Math ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')