with open('2016\\Input\\2016-06.txt') as input:
    text_input = ['','','','','','','','']
    for line in input:
        text = line.strip()
        for i in range(len(text)):
            text_input[i] += text[i]

def count_characters(string, part):
    char_count = {}

    for c in string:
        if c in char_count:
            char_count[c] += 1
        else:
            char_count[c] = 1
    
    return max(char_count, key=char_count.get) if part else min(char_count, key=char_count.get)

def decypher(code_list, part):
    code = ''

    for s in code_list:
        code += count_characters(s, not part)

    return code

part_one = decypher(text_input, False)
part_two = decypher(text_input, True)

print('--- Day 6: Signals and Noise ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')