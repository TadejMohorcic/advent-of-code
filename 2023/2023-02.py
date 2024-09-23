with open('2023/Input/2023-02.txt') as input:
    cubes = []

    for line in input:
        game_id, games = line.strip().split(':')
        game_id = int(game_id.split()[1])
        games = games.split(';')
        cubes.append((game_id, games))

def valid_games(cube_list, part):
    s = 0

    for l in cube_list:
        index, games = l
        r, g, b = 0, 0, 0

        for game in games:
            cubes = game.split(',')
            for cube in cubes:
                if 'red' in cube:
                    r = max(r, int(cube.split()[0]))
                elif 'green' in cube:
                    g = max(g, int(cube.split()[0]))
                else:
                    b = max(b, int(cube.split()[0]))

        if part:
            s += r * g * b
        else:
            if r <= 12 and g <= 13 and b <= 14:
                s += index
    
    return s

part_one = valid_games(cubes, False)
part_two = valid_games(cubes, True)

print('--- Day 2: Cube Conundrum ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')