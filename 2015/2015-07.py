import re

with open('2015/Input/2015-07.txt') as input:
    instructions = {}

    for line in input:
        split_line = re.split('->', line.strip())
        target = split_line[1].strip()
        command = re.split(' ', split_line[0].strip())
        instructions[target] = command

def get_value(key, values):
    try:
        return int(key)
    except ValueError:
        pass

    if key in values:
        return values[key]    
    else:
        commands = instructions[key]
        if len(commands) == 1:
            if commands[0].isdigit():
                result = int(commands[0])
            else:
                result = get_value(commands[0], values)
        elif len(commands) == 2:
            result = ~get_value(commands[1], values) & (2**16 - 1)
        else:
            if commands[1] == 'AND':
                result = get_value(commands[0], values) & get_value(commands[2], values)
            if commands[1] == 'OR':
                result = get_value(commands[0], values) | get_value(commands[2], values)
            if commands[1] == 'LSHIFT':
                result = get_value(commands[0], values) << int(commands[2])
            if commands[1] == 'RSHIFT':
                result = get_value(commands[0], values) >> int(commands[2])
            
        values[key] = result

        return result

part_one = get_value('a', {})
instructions['b'] = [f'{part_one}']
part_two = get_value('a', {})

print('--- Day 7: Some Assembly Required ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')