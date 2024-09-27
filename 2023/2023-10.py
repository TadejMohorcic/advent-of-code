with open('2023/Input/2023-10.txt') as input:
    maze = []

    for line in input:
        maze.append(line.strip())

def get_start(maze_list):
    i = 0

    for row in maze_list:
        j = 0

        if 'S' in row:
            for col in row:
                if col == 'S':
                    return (i, j)
                j += 1 

        i += 1 

def move_around(maze_list):
    n1, n2 = len(maze_list), len(maze_list[0])
    x, y = get_start(maze_list)
    queue = [(x, y)]
    seen = set()

    while queue != []:
        x, y = queue.pop()

        if (x, y) in seen:
            continue
        else:
            seen.add((x, y))
        
        match maze_list[x][y]:
            case 'S':
                if x + 1 < n1 and maze_list[x + 1][y] in 'LJ|':
                    queue.append((x + 1, y))
                if x - 1 >= 0 and maze_list[x - 1][y] in 'F7|':
                    queue.append((x - 1, y))
                if y + 1 < n2 and maze_list[x][y + 1] in 'J7-':
                    queue.append((x, y + 1))
                if y - 1 >= 0 and maze_list[x][y - 1] in 'LF-':
                    queue.append((x, y - 1))
            case '|':
                if x + 1 < n1 and maze_list[x + 1][y] in 'LJ|':
                    queue.append((x + 1, y))
                if x - 1 >= 0 and maze_list[x - 1][y] in 'F7|':
                    queue.append((x - 1, y))
            case '-':
                if y + 1 < n2 and maze_list[x][y + 1] in 'J7-':
                    queue.append((x, y + 1))
                if y - 1 >= 0 and maze_list[x][y - 1] in 'LF-':
                    queue.append((x, y - 1))
            case 'F':
                if x + 1 < n1 and maze_list[x + 1][y] in 'LJ|':
                    queue.append((x + 1, y))
                if y + 1 < n2 and maze_list[x][y + 1] in 'J7-':
                    queue.append((x, y + 1))
            case '7':
                if x + 1 < n1 and maze_list[x + 1][y] in 'LJ|':
                    queue.append((x + 1, y))
                if y - 1 >= 0 and maze_list[x][y - 1] in 'LF-':
                    queue.append((x, y - 1))
            case 'L':
                if x - 1 >= 0 and maze_list[x - 1][y] in 'F7|':
                    queue.append((x - 1, y))
                if y + 1 < n2 and maze_list[x][y + 1] in 'J7-':
                    queue.append((x, y + 1))
            case 'J':
                if x - 1 >= 0 and maze_list[x - 1][y] in 'F7|':
                    queue.append((x - 1, y))
                if y - 1 >= 0 and maze_list[x][y - 1] in 'LF-':
                    queue.append((x, y - 1))

    return len(seen) // 2, seen

def update_maze(maze_list, special, part):
    new_maze = []
    i = 0

    for r in maze_list:
        new_line = ''
        j = 0

        for char in maze_list[i]:
            if part:
                if (i, j) not in special and char == '.':
                    new_line += '~'
                else:
                    new_line += char
            else:
                if (i, j) in special:
                    new_line += '│' if char == '|' else ''
                    new_line += '─' if char == '-' else ''
                    new_line += '┐' if char == '7' else ''
                    new_line += '┌' if char == 'F' else ''
                    new_line += '┘' if char == 'J' else ''
                    new_line += '└' if char == 'L' else ''
                    new_line += 'S' if char == 'S' else ''
                else:
                    new_line += '.'

            j += 1
        
        new_maze.append(new_line)
        i += 1

    return new_maze

def count_inside(maze_list):
    inside = set()
    res = 0
    i = 0

    for _ in maze_list:
        inside_maze = False
        opener = ''
        j = 0

        for c in maze_list[i]:
            if c == '.':
                res += 1 if inside_maze else 0
                if inside_maze:
                    inside.add((i, j))

            elif c in 'S┌└':
                inside_maze = not inside_maze
                opener = c
            elif c in '┘┐':
                if c == '┘' and opener in 'S└' or c == '┐' and opener in '┌':
                    inside_maze = not inside_maze
                opener = ''
            elif c == '│':
                inside_maze = not inside_maze

            j += 1
        
        i += 1

    return res, inside

part_one, maze_indices = move_around(maze)

updated_maze = update_maze(maze, maze_indices, False)
part_two, inside = count_inside(updated_maze)

final_maze = update_maze(updated_maze, inside, True)

for line in final_maze:
    print(line)

print('\n')

print('--- Day 10: Pipe Maze ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')