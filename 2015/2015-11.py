letters = list('abcdefghijklmnopqrstuvwxyz')

with open('2015/Input/2015-11.txt') as input:
    text = list(input.readline().strip())

def increment(char_list):
    char_list[-1] += 1
    for i in range(len(char_list)):
        if char_list[-1 - i] > 25:
            char_list[-1 - i] = char_list[-1 - i] % 26
            try:
                char_list[-2 - i] += 1
            except IndexError:
                continue
    return char_list

def check_letters(char_list):
    for i in range(len(char_list) - 2):
        if char_list[i + 1] - char_list[i] == 1 and char_list[i + 2] - char_list[i + 1] == 1:
            return True
    return False

def illegal_letters(char_list):
    if 8 in char_list or 11 in char_list or 14 in char_list:
        return False
    else:
        return True
    
def double_letters(char_list):
    i = 0
    counter = 0

    while i < len(char_list) - 1:
        if char_list[i] == char_list[i+1]:
            counter += 1
            i += 2
        else:
            i += 1
    
    if counter > 1:
        return True
    else:
        return False

def new_password(char_list):
    while True:
        increment(char_list)
        t1 = check_letters(char_list)
        t2 = illegal_letters(char_list)
        t3 = double_letters(char_list)
        if t1 and t2 and t3 == True:
            return char_list
        
def char_to_digit(char):
    return letters.index(char)

def list_to_string(char_list):
    string = ''
    for c in char_list:
        string += c
    return string

id_list = list(map(char_to_digit, text))

password_list = new_password(id_list)
password = list(map(lambda x: letters[x], password_list))
part_one = list_to_string(password)

password_list = new_password(password_list)
password = list(map(lambda x: letters[x], password_list))
part_two = list_to_string(password)

print('--- Day 11: Corporate Policy ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')