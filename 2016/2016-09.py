with open('2016/Input/2016-09.txt') as input:
    text = input.readline().strip()

def get_range_and_repetition(string):
    split_string = string.split(')')[0]
    str_length = len(split_string)
    numbers = [int(x) for x in split_string.split('x')]

    return numbers[0], numbers[1], str_length + 2

def evaluate_string(string, part):
    i = 0
    length = 0
    
    while i < len(string):
        current_char = string[i]
        if current_char == '(':
            l, r, shift = get_range_and_repetition(string[(i + 1):])
            length += r * evaluate_string(string[i+shift:i+shift+l], part) if part else r * l
            i += shift + l
        else:
            length += 1
            i += 1

    return length

part_one = evaluate_string(text, False)
part_two = evaluate_string(text, True)

print('--- Day 9: Explosives in Cyberspace ---')
print(f' -  Part one solution: {part_one}')
print(f' -  Part two solution: {part_two}')