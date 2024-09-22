import numpy as np

with open('2015/Input/2015-20.txt') as input:
    number = int(input.readline().strip()) // 10

def update(num, part):
    divisors = np.zeros(num)

    for i in range(1,num):
        counter = 0
        for j in range(i,num,i):
            if part:
                if counter <= 50:
                    divisors[j] += i * 11
                    counter += 1
            else:
                divisors[j] += i * 10

        if divisors[i] >= num * 10:
            return i

part_one = update(number, False)
part_two = update(number, True)

print('--- Day 20: Infinite Elves and Infinite Houses ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')