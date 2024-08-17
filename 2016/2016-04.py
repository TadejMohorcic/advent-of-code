letters = list('abcdefghijklmnopqrstuvwxyz')

with open('2016\\Input\\2016-04.txt') as input:
    rooms = []

    for line in input:
        text = line.strip().split('-')

        chars = ''
        for i in range(len(text) - 1):
            chars += text[i]

        last = text[-1].split('[')
        identification = int(last[0])
        common_letters = last[1][:-1]

        rooms.append((chars, identification, common_letters))

def count_characters(string):
    char_dict = {}

    for c in string:
        if c not in char_dict:
            char_dict[c] = 1
        else:
            char_dict[c] += 1

    char_list = [(key, char_dict[key]) for key in char_dict]
    char_list.sort(key=lambda x: (-x[1], x[0]))

    commons = [x for (x, _) in char_list]

    return commons[0:5]

def shift_cypher(string, number):
    new_string = ''

    for c in string:
        new_string += letters[(letters.index(c) + number) % 26]

    return new_string

def check_valid_room(room_list):
    count = 0

    for r in room_list:
        name, sector_id, commons = r[0], r[1], r[2]
        most_common = count_characters(name)
        if most_common == list(commons):
            count += sector_id

            decyphered_string = shift_cypher(name, sector_id)
            if decyphered_string == 'northpoleobjectstorage':
                index = sector_id

    return count, index

part_one, part_two = check_valid_room(rooms)

print('--- Day 4: Security Through Obscurity ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')