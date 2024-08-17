import hashlib

with open('2016\\Input\\2016-05.txt') as input:
    door_id = input.readline().strip()

def list_to_string(list):
    string = ''

    for l in list:
        string += l

    return string

def find_password(door, part):
    password = ['-','-','-','-','-','-','-','-']
    found = 0
    test_number = 1

    while found < 8:
        output = hashlib.md5((door + str(test_number)).encode()).hexdigest()

        if output[0:5] == '00000':

            if part:
                position = output[5]
                if position.isdigit():
                    if 0 <= int(position) <= 7:
                        if password[int(position)] == '-':
                            password[int(position)] = output[6]
                            found += 1
            else:
                password[found] = output[5]
                found += 1

        test_number += 1
    
    return password

part_one = list_to_string(find_password(door_id, False))
part_two = list_to_string(find_password(door_id, True))

print('--- Day 5: How About a Nice Game of Chess? ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')