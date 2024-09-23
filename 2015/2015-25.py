with open('2015/Input/2015-25.txt') as input:
    text = input.readline().strip().split()
    row = int(text[-3][:-1])
    column = int(text[-1][:-1])

start = 20151125
mul = 252533
div = 33554393

def grid_to_row(r, c):
    return c + (c + r - 1)*(c + r - 2) // 2

def modular_exponent(base, exp, mod):
    if mod == 1:
        return 0
    
    res = 1
    base = base % mod
    while exp > 0:
        if exp % 2 == 1:
            res = (res * base) % mod
        exp = exp >> 1
        base = (base * base) % mod

    return res

end = grid_to_row(row, column)
part_one = (start * modular_exponent(mul, end - 1, div)) % div

    
print('--- Day 25: Let It Snow ---')
print(f' -  Part one solution: {part_one}.')