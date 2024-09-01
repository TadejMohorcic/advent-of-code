import hashlib

with open('2016/Input/2016-14.txt') as input:
    salt = input.readline().strip()

def valid_key(key):
    for i in range(len(key) - 2):
        if key[i] == key[i+1] == key[i+2]:
            return key[i]
    return None

def find_keys(string, part):
    index = 0
    found_keys = set()
    potential_keys = []

    while len(found_keys) < 64:
        output = hashlib.md5((string + str(index)).encode()).hexdigest()

        if part:
            for i in range(2016):
                output = hashlib.md5(output.encode()).hexdigest()

        for c, n in potential_keys:
            if 5 * c in output and index - n <= 1000:
                found_keys.add(n)

        char = valid_key(output)

        if char != None:
            potential_keys.append((char, index))

        index += 1

    return sorted(found_keys)[63]

part_one = find_keys(salt, False)
part_two = find_keys(salt, True)

print('--- Day 14: One-Time Pad ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')