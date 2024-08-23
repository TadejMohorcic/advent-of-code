with open('2015/Input/2015-01.txt') as input:
    brackets = input.readline().strip()

def calculate_floor(string, part):
    floor = 0

    for i in range(len(string)):
        if string[i] == ')':
            floor -= 1
            if part and floor < 0:
                return i + 1
        else:
            floor += 1
    
    return floor

part_one = calculate_floor(brackets, False)
part_two = calculate_floor(brackets, True)

print('--- Day 1: Not Quite Lisp ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')