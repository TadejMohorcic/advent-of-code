with open('2015/Input/2015-03.txt') as input:
    directions = input.readline().strip()

def get_directions(string):
    match string:
        case 'v':
            return (0, -1)
        case '>':
            return (1, 0)
        case '<':
            return (-1, 0)
        case '^':
            return (0, 1)
        
def present_delivery(directions, part):
    houses = set()
    houses.add((0, 0))

    santa_position = (0, 0)
    if part:
        robo_position = (0, 0)

    step = 2 if part else 1

    for i in range(0, len(directions), step):
        if part:
            robo_direction = get_directions(directions[i + 1])
            robo_position = (robo_position[0] + robo_direction[0], robo_position[1] + robo_direction[1])
            houses.add(robo_position)

        santa_direction = get_directions(directions[i])
        santa_position = (santa_position[0] + santa_direction[0], santa_position[1] + santa_direction[1])
        houses.add(santa_position)

    return len(houses)

part_one = present_delivery(directions, False)
part_two = present_delivery(directions, True)

print('--- Day 3: Perfectly Spherical Houses in a Vacuum ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')