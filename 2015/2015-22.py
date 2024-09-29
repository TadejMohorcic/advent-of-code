import numpy as np

# boss = (hp, damage)
with open('2015/Input/2015-22.txt') as input:
    hp = int(input.readline().strip().split()[-1])
    dmg = int(input.readline().strip().split()[-1])

boss = (hp, dmg)

# player = (hp, mana)
player = (50, 500)

# spell = [cost, damage, heal, armor, mana regen, turns, index]
spells = [[53, 4, 0, 0, 0, 1, 0], [73, 2, 2, 0, 0, 1, 1], [113, 0, 0, 7, 0, 6, 2], [173, 3, 0, 0, 0, 6, 3], [229, 0, 0, 0, 101, 5, 4]]

def simulate(player, boss, active_spells, player_turn, mana_spent):
    player_hp, player_mana = player[0], player[1]
    boss_hp, boss_dmg = boss[0], boss[1]
    player_armor = 0

    if part_two and player_turn:
        player_hp -= 1
        if player_hp <= 0:
            return False

    next_spells = []

    for spell in active_spells:
        boss_hp -= spell[1]
        player_hp += spell[2]
        player_armor += spell[3]
        player_mana += spell[4]
        new_spell = [spell[0], spell[1], spell[2], spell[3], spell[4], spell[5] - 1, spell[6]]
        if new_spell[5] > 0:
            next_spells.append(new_spell)

    if boss_hp <= 0:
        global least_mana
        if mana_spent < least_mana:
            least_mana = mana_spent
        return True
    
    if mana_spent >= least_mana:
        return False

    if player_turn:
        active = [x for (_, _, _, _, _, _, x) in next_spells]

        for i in range(5):
            cost = spells[i][0]
            if cost <= player_mana and not i in active:
                active_copy = next_spells.copy()
                active_copy.append(spells[i])
                simulate((player_hp, player_mana - cost), (boss_hp, boss_dmg), active_copy, False, mana_spent + cost)
    else:
        damage = max(boss_dmg - player_armor, 1)
        player_hp -= damage
        
        if player_hp > 0:
            simulate((player_hp, player_mana), (boss_hp, boss_dmg), next_spells, True, mana_spent)

part_two = False
least_mana = np.inf
simulate(player, boss, [], True, 0)
part_one = least_mana

part_two = True
least_mana = np.inf
simulate(player, boss, [], True, 0)
part_two = least_mana

print('--- Day 22: Wizard Simulator 20XX ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')