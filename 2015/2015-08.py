with open('2015/Input/2015-08.txt') as input:
    lines = []

    for line in input:
        stripped = line.strip()
        lines.append(stripped)

def count_characters(string):
    count = 0
    i = 0

    while i < len(string):
        if string[i] == '\\':
            if string[i + 1] == 'x':
                i += 4
            else:
                i += 2
        else:
            i += 1
        count += 1

    return count

def encode_characters(string):
    encoded_string = '"'

    for char in string:
        if char == '\"':
            encoded_string += '\\\"'
        elif char == '\\':
            encoded_string += '\\\\'
        else:
            encoded_string += char

    return encoded_string + '"'

def string_length(string_list):
    len1, len2 = 0, 0

    for s in string_list:
        length = len(s)
        len1 += length - count_characters(s) + 2
        len2 += len(encode_characters(s)) - length

    return len1, len2

part_one, part_two = string_length(lines)

print('--- Day 8: Matchsticks ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')