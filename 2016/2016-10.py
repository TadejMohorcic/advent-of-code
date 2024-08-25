import copy

with open('2016/Input/2016-10.txt') as input:
    bot_instructions = {}
    bot_values = {}

    for line in input:
        bot = line.strip().split()
        if bot[0] == 'bot':
            bot_id = int(bot[1])
            bot_instructions[bot_id] = [bot[5], int(bot[6]), bot[-2], int(bot[-1])]
        else:
            bot_id = int(bot[-1])
            bot_value = int(bot[1])
            if bot_id in bot_values:
                bot_values[bot_id].append(bot_value)
            else:
                bot_values[bot_id] = [bot_value]

def update_dict(key, values_dict, instructions_dict, output_dict):
    values = values_dict.pop(key)
    min_value, max_value = min(values), max(values)
    low_type, low, high_type, high = instructions_dict[key]
    next_id = []

    if low_type == 'bot':
        if low in values_dict:
            values_dict[low].append(min_value)
            next_id.append(low)
        else:
            values_dict[low] = [min_value]
    else:
        output_dict[low] = min_value

    if high_type == 'bot':
        if high in values_dict:
            values_dict[high].append(max_value)
            next_id.append(high)
        else:
            values_dict[high] = [max_value]
    else:
        output_dict[high] = max_value

    return next_id, values_dict, output_dict

def get_result(start_id, values_dict, instructions_dict, part):
    output = {}
    next_id = [start_id]
    values = copy.deepcopy(values_dict)
    
    while True:
        current_id = next_id.pop(0)

        if part:
            if 0 in output and 1 in output and 2 in output:
                return output[0] * output[1] * output[2]
        else:
            vals = values[current_id]
            if min(vals) == 17 and max(vals) == 61:
                return current_id

        next_ids, values, output = update_dict(current_id, values, instructions_dict, output)
        next_id += next_ids
        
part_one = get_result(13, bot_values, bot_instructions, False)
part_two = get_result(13, bot_values, bot_instructions, True)

print('--- Day 10: Balance Bots ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part one solution: {part_two}.')