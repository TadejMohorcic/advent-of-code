with open('2016/Input/2016-16.txt') as input:
    text = input.readline().strip()

def increase_length(string):
    reversed_list = [abs(int(x) - 1) for x in string]
    reversed_list.reverse()
    joined = list(string) + ['0'] + [str(x) for x in reversed_list]
    
    return ''.join(joined)

def calculate_checksum(string):
    i = 0
    new_string = ''

    while i < len(string):
        a, b = string[i], string[i + 1]
        new_string += '1' if a == b else '0'
        i += 2

    return new_string

def dragon_curve(string, length):
    while len(string) < length:
        string = increase_length(string)

    string = string[:length]

    while len(string) % 2 == 0:
        string = calculate_checksum(string)

    return string

part_one = dragon_curve(text, 272)
part_two = dragon_curve(text, 35651584)

print('--- Day 16: Dragon Checksum ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')