import re

with open('2015/Input/2015-19.txt') as input:
    instructions = {}
    replace = True

    for line in input:
        text = line.strip().split()
        if text == []:
            replace = False
            continue

        if replace:
            if text[0] not in instructions:
                instructions[text[0]] = []
                instructions[text[0]].append(text[2])
            else:
                instructions[text[0]].append(text[2])
        else:
            molecule = text[0]

def calculate_possibilities(molecule, instruction_dict):
    possibilities = set()

    for i in range(len(molecule)):
        for j in range(2):
            symbol = molecule[i:i + j + 1]
            if symbol in instruction_dict:
                for c in instruction_dict[symbol]:
                    new_molecule = molecule[:i] + c + molecule[i + j + 1:]
                    possibilities.add(new_molecule)

    return len(possibilities)

# found on reddit...
def calculate_steps(molecule):
    elements = re.findall('[A-Z][^A-Z]*', molecule)
    n = len(elements)
    m1, m2 = 0, 0

    for e in elements:
        if e == 'Ar' or e == 'Rn':
            m1 += 1
        elif e == 'Y':
            m2 += 1
    
    return n - m1 - 2*m2 - 1

part_one = calculate_possibilities(molecule, instructions)
part_two = calculate_steps(molecule)

print('--- Day 19: Medicine for Rudolph ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')