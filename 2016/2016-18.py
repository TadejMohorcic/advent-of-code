with open('2016/Input/2016-18.txt') as input:
    text = input.readline().strip()

def calculate_next_line(string):
    new_string = ''
    empty_space = 0

    for i in range(len(string)):
        l = string[i - 1] if i - 1 >= 0 else '.'
        r = string[i + 1] if i + 1 < len(string) else '.'

        if l != r:
            new_string += '^'
            empty_space += 0
        else:
            new_string += '.'
            empty_space += 1

    return new_string, empty_space

def update_line(string, n):
    safe_tiles = 0

    for c in string:
        if c == '.':
            safe_tiles += 1

    for i in range(n):
        string, s = calculate_next_line(string)
        safe_tiles += s

    return safe_tiles

part_one = update_line(text, 39)
part_two = update_line(text, 399999)

print('--- Day 18: Like a Rogue ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')