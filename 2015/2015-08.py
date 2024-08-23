part_one = 0
part_two = 0

def count_strings(s):
    count = 0
    i = 0
    
    while i < len(s):
        if s[i] == '\\':
            if s[i+1] == "x":
                i += 4
            else:
                i += 2
        else:
            i += 1
        count += 1
    
    return count

def encode_string(s):
    encoded_s = '"'
    for c in s:
        if c == '\"':
            encoded_s += '\\\"'
        elif c == '\\':
            encoded_s += '\\\\'
        else:
            encoded_s += c

    encoded_s += '"'

    return encoded_s


with open('2015\\Input//day08.txt') as input:
    for line in input:
        stripped = line.strip()
        encode = encode_string(stripped)

        n1 = len(stripped)
        n2 = len(encode)

        c1 = count_strings(stripped) - 2

        part_one += n1 - c1
        part_two += n2 - n1

print('--- Day 8: Matchsticks ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')