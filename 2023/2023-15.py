with open('2023/Input/2023-15.txt') as input:
    for line in input:
        lens = line.strip().split(',')

def hash_string(string):
    res = 0

    for char in string:
        res = ((res + ord(char)) * 17) % 256

    return res

def calculate_focusing_power(lens_dict):
    res = 0
    i = 1

    for key in lens_dict:
        res += i * lens_dict[key]
        i += 1

    return res

def lens_dictionary(string_list):
    lens_dict = {}

    for string in string_list:

        if string[-1] in '123456789':
            label, symbol, lens = string[:-2], string[-2], int(string[-1])
        else:
            label, symbol = string[:-1], string[-1]

        box_index = hash_string(label)

        if box_index in lens_dict:
            if symbol == '=':
                    lens_dict[box_index][label] = lens
            else:
                if label in lens_dict[box_index]:
                    lens_dict[box_index].pop(label)

        else:
            if symbol == '=':
                lens_dict[box_index] = {label: lens}

    return lens_dict

def focusing_power(string_list, part):
    res = 0

    if part:
        lens_dict = lens_dictionary(string_list)

        for box in lens_dict:
            res += (box + 1) * calculate_focusing_power(lens_dict[box])

    else:
        for string in string_list:
            res += hash_string(string)

    return res

part_one = focusing_power(lens, False)
part_two = focusing_power(lens, True)

print('--- Day 15: Lens Library ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')