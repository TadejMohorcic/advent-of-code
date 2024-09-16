with open('2016/Input/2016-19.txt') as input:
    number = int(input.readline().strip())

def calculate_winner(n, part):
    if part:
        i = 0

        while n >= 3 ** i:
            i += 1
        k = i - 1

        if n == 3 ** k:
            result = n

        elif n - 2 * 3 ** k < 0:
            result = n - 3 ** k

        else:
            result = 2 * number - 3**(k + 1)
    else: 
        to_binary = bin(n)[2:]
        new_n = to_binary[1:] + to_binary[0]
        result = int(new_n, 2)

    return result
    
part_one = calculate_winner(number, False)
part_two = calculate_winner(number, True)

print('--- Day 19: An Elephant Named Joseph ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')