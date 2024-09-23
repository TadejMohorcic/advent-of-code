import re

numbers = '(?=(\\d|one|two|three|four|five|six|seven|eight|nine))'
num_list = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']

with open('2023/Input/2023-01.txt') as input:
    text = []
    for line in input:
        text.append(line.strip())

def calibration_values(text_list, part):
    s = 0

    for line in text_list:
        digits = ''
        if part:
            for num in re.findall(numbers, line):
                digits += str(num_list.index(num) + 1) if num in num_list else num
        else:
            for c in line:
                if c.isdigit():
                    digits += c

        s += int(digits[0]) * 10 + int(digits[-1])

    return s
    
part_one = calibration_values(text, False)
part_two = calibration_values(text, True)

print('--- Day 1: Trebuchet?! ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')