with open('2023/Input/2023-09.txt') as input:
    numbers = []

    for line in input:
        number_line = [int(n) for n in line.strip().split()]
        numbers.append(number_line)

def calculate_history(number_list):
    n = len(number_list)
    history = {n: number_list}

    while True:
        next_row = []

        for i in range(1, len(history[n])):
            next_row.append(history[n][i] - history[n][i - 1])

        n -= 1
        history[n] = next_row

        if next_row == n * [0]:
            return history
        
def extrapolated_values(number_list, part):
    res = 0
    
    for numbers in number_list:
        history = calculate_history(numbers)
        n, m = len(numbers), len(history)

        for i in range(n - m + 1, n):
            if part:
                history[i + 1].insert(0, history[i + 1][0] - history[i][0])
            else:
                history[i + 1].append(history[i][-1] + history[i + 1][-1])

        res += history[n][0] if part else history[n][-1]
    
    return res

part_one = extrapolated_values(numbers, False)
part_two = extrapolated_values(numbers, True)

print('--- Day 9: Mirage Maintenance ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')