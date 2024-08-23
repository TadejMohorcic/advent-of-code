import re

with open('2015/Input/2015-05.txt') as input:
    strings = []
    
    for line in input:
        strings.append(line.strip())

def check_string(string, part):
    if part:
        repeats = re.findall(r'(..).*\1', string)
        repeats_between = re.findall(r'(.).\1', string)
        return 1 if len(repeats) >= 1 and len(repeats_between) >= 1 else 0
    else:
        vowels = re.findall(r'[aeiou]', string)
        repeats = re.findall(r'(.)\1', string)
        forbidden = re.findall(r'ab|cd|pq|xy', string)
        return 1 if len(vowels) >= 3 and len(repeats) >= 1 and len(forbidden) == 0 else 0

def check_strings(string_list, part):
    count = 0

    for s in string_list:
        count += check_string(s, part)

    return count

part_one = check_strings(strings, False)
part_two = check_strings(strings, True)

print('--- Day 5: Doesnt He Have Intern-Elves For This? ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')