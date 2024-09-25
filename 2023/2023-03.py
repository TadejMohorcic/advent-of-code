import re

with open('2023/Input/2023-03.txt') as input:
    numbers = {}
    text = []

    row = 1
    for line in input:
        line = line.strip()
        text.append(line)
        number = [int(n) for n in re.findall(r'\d+', line)]
        index = [m.span() for m in re.finditer(r'\d+', line)]
        numbers[row] = list(zip(number, index))
        row += 1

def find_chars(text_list, chars):
    char_dict = {}
    row = 1

    for line in text_list:
        col = 1
        for char in line:
            if char in chars:
                char_dict[row * 140 + col] = []
            col += 1
        row += 1

    return char_dict

def find_adjecent(char_dict, number_dict):
    for key in char_dict:
        row, col = key // 140, key % 140
        for i in range(row - 1, row + 2):
            for num in number_dict[i]:
                if abs(num[1][0] + 1 - col) <= 1 or abs(num[1][1] - col) <= 1:
                    char_dict[key].append(num[0])
    return char_dict

def calculate_rations(text_list, chars, number_dict, part):
    char_dict = find_chars(text_list, chars)
    char_dict = find_adjecent(char_dict, number_dict)

    res = 0

    for key in char_dict:
        if part:
            if len(char_dict[key]) == 2:
                res += char_dict[key][0] * char_dict[key][1]
        else:
            res += sum(char_dict[key])
    
    return res

part_one = calculate_rations(text, '@#$%&/=+-*', numbers, False)
part_two = calculate_rations(text, '*', numbers, True)

print('--- Day 3: Gear Ratios ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')