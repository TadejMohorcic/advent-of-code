import hashlib

with open('2015/Input/2015-04.txt') as input:
    secret_key = input.readline().strip()

def find_hash(string, part):
    found = False
    test_number = 1

    decimals = 6 if part else 5

    while not found:
        output = hashlib.md5((string + str(test_number)).encode()).hexdigest()
        if output[0:decimals] == decimals * '0':
            return test_number
        else:
            test_number += 1

part_one = find_hash(secret_key, False)
part_two = find_hash(secret_key, True)

print('--- Day 4: The Ideal Stocking Stuffer ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')