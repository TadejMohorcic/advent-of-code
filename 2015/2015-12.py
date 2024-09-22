import json

with open('2015/Input/2015-12.txt') as input:
    json_t = json.loads(input.readline().strip())

def sum_numbers(json_data, part):

    if type(json_data) == int:
        return json_data
    
    elif type(json_data) == list:
        return sum([sum_numbers(n, part) for n in json_data])
    
    elif type(json_data) == dict:
        if part and 'red' in json_data.values():
            return 0
        else:
            return sum_numbers(list(json_data.values()), part)
        
    else:
        return 0

part_one = sum_numbers(json_t, False)
part_two = sum_numbers(json_t, True)

print('--- Day 12: JSAbacusFramework.io ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')