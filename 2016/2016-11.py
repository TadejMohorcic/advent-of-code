with open('2016/Input/2016-11.txt') as input:
    floors = []

    for line in input:
        current_floor = 0
        text = line.strip().split(' a ')
        for t in text:
            element_type = t.split(' ')[1]
            if element_type in ['generator,', 'generator', 'generator.', 'microchip,', 'microchip', 'microchip.']:
                current_floor += 1
        floors.append(current_floor)

def number_of_steps(floor_list):
    floors_copy = floor_list.copy()
    
    steps = 0
    current_floor = 0
    n = len(floors_copy) - 1
    elements = sum(floors_copy)

    while floors_copy[n] != elements:
        current_elements = floors_copy[current_floor]
        steps += 2 * (current_elements - 1) - 1
        floors_copy[current_floor] = 0
        current_floor += 1
        floors_copy[current_floor] += current_elements
    
    return steps

def get_number(floor_list, part):
    floors[0] += 4 if part else 0
    return number_of_steps(floor_list)

part_one = get_number(floors, False)
part_two = get_number(floors, True)

print('--- Day 11: Radioisotope Thermoelectric Generators ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')