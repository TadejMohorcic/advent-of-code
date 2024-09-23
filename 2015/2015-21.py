with open('2015/Input/2015-21.txt') as input:
    storage = []
    weapons = []
    boss = []

    for line in input:
        txt = line.strip().split()
        if txt == []:
            storage.append(weapons)
            weapons = []
        else:
            if len(txt) == 4:
                try:
                    weapons.append((int(txt[1]), int(txt[2]), int(txt[3])))
                except:
                    continue
            elif len(txt) == 5:
                weapons.append((int(txt[1][-1]), int(txt[2]), int(txt[3]), int(txt[4])))
            else:
                boss.append(int(txt[-1]))

def calculate_damage(player, boss):
    player_damage = player[1] - boss[2] if player[1] - boss[2] > 0 else 1
    boss_damage = boss[1] - player[2] if boss[1] - player[2] > 0 else 1

    player_turns = boss[0] // player_damage
    boss_turns = player[0] // boss_damage

    if boss[0] % player_damage != 0:
        player_turns += 1

    if player[0] % boss_damage != 0:
        boss_turns += 1

    return player_turns <= boss_turns

def win_cost(storage, boss, part):
    weapons, armor, rings = storage[0], storage[1], storage[2]
    armor.append((0,0,0))
    rings.append((0,0,0,0))
    rings.append((0,0,0,0))

    win_cost = 0 if part else 1000

    for w in weapons:
        for a in armor:
            for r in rings:
                rings_2 = rings.copy()
                rings_2.remove(r)
                for p in rings_2:
                    cost = w[0] + a[0] + r[1] + p[1]
                    player = [100, w[1] + a[1] + r[2] + p[2], w[2] + a[2] + r[3] + p[3]]
                    if part:
                        if cost > win_cost:
                            if not calculate_damage(player, boss):
                                win_cost = cost
                    else:
                        if cost < win_cost:
                            if calculate_damage(player, boss):
                                win_cost = cost
    
    return win_cost

part_one = win_cost(storage, boss, False)
part_two = win_cost(storage, boss, True)

print('--- Day 21: RPG Simulator 20XX ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')