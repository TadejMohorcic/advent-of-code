with open('2015/Input/2015-15.txt') as input:
    ingredients = []

    for line in input:
        text = line.strip().split()
        capacity, durability, flavour, texture, calories = text[2][:-1], text[4][:-1], text[6][:-1], text[8][:-1], text[10]
        ingredients.append([int(capacity), int(durability), int(flavour), int(texture), int(calories)])


def calculate_score(ingredient_list, part):
    max_score = 0

    for i in range(101):
        for j in range(101 - i):
            for k in range(101 - i - j):
                l = 100 - i - j - k
                i1 = list(map(lambda x: i*x, ingredient_list[0]))
                i2 = list(map(lambda x: j*x, ingredient_list[1]))
                i3 = list(map(lambda x: k*x, ingredient_list[2]))
                i4 = list(map(lambda x: l*x, ingredient_list[3]))

                s = list(map(lambda x, y, z, w: x + y + z + w, i1, i2, i3, i4))
                s = list(map(lambda x: x if x > 0 else 0, s))
                if part:
                    if s[4] == 500:
                        score = s[0] * s[1] * s[2] * s[3]
                        if score > max_score:
                            max_score = score
                else:
                    score = s[0] * s[1] * s[2] * s[3]
                    if score > max_score:
                            max_score = score

    return max_score


part_one = calculate_score(ingredients, False)
part_two = calculate_score(ingredients, True)

print('--- Day 15: Science for Hungry People ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')