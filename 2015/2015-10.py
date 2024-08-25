with open('2015/Input/2015-10.txt') as input:
    text = input.readline().strip()

def look_and_see(string, steps):
    counter = 0

    while counter <= steps:
        new_string = ''
        prev_digit = ''
        curr_count = 0

        for i in range(len(string)):
            curr_digit = string[i]
            if prev_digit == '' or prev_digit == curr_digit:
                prev_digit = curr_digit
                curr_count += 1
            else:
                new_string += str(curr_count) + prev_digit
                prev_digit = curr_digit
                curr_count = 1

        new_string += str(curr_count) + prev_digit

        counter += 1
        string = new_string

    return new_string

part_one = len(look_and_see(text, 39))
part_two = len(look_and_see(text, 49))

print('--- Day 10: Elves Look, Elves Say ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')