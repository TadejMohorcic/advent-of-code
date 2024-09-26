with open('2023/Input/2023-07.txt') as input:
    cards = []

    for line in input:
        hand, bid = line.strip().split()
        cards.append((hand, int(bid)))

def compare_cards(card_1, card_2, part):
    card_list = list('AKQT98765432J')if part else list('AKQJT98765432')

    for i in range(len(card_1)):
        if card_list.index(card_1[i]) < card_list.index(card_2[i]):
            return True
        elif card_list.index(card_1[i]) > card_list.index(card_2[i]):
            return False
        
def insertion_sort(card_list, part):
    i = 1

    while i < len(card_list):
        j = i

        while j > 0 and compare_cards(card_list[j][0], card_list[j - 1][0], part):
            current = card_list[j]
            card_list[j] = card_list[j - 1]
            card_list[j - 1] = current
            j -= 1
        
        i += 1

    return card_list

def get_hand_type(hand, part):
    seen = ''
    hand_type = []
    jokers = 0

    for c in hand:
        if c in seen:
            continue
        else:
            if c == 'J' and part:
                jokers += hand.count(c)
                seen += c
            else:
                hand_type.append(hand.count(c))
                seen += c

    hand_type.sort()

    if part:
        if hand_type == []:
            return [jokers]
        else:
            hand_type[-1] += jokers

    return hand_type[::-1]

def get_card_type(card_list, part):
    hand_dict = {'five_of_a_kind': [], 'four_of_a_kind': [], 'full_house': [], 'three_of_a_kind': [], 'two_pair': [], 'one_pair': [], 'high_card': []}

    for card, bid in card_list:
        hand_type = get_hand_type(card, part)
        match hand_type[0]:
            case 5:
                hand_dict['five_of_a_kind'].append((card, bid))
            case 4:
                hand_dict['four_of_a_kind'].append((card, bid))
            case 3:
                hand_dict['full_house'].append((card, bid)) if hand_type[1] == 2 else hand_dict['three_of_a_kind'].append((card, bid))
            case 2:
                hand_dict['two_pair'].append((card, bid)) if hand_type[1] == 2 else hand_dict['one_pair'].append((card, bid))
            case 1:
                hand_dict['high_card'].append((card, bid))

    return hand_dict

def calculate_winnings(card_list, part):
    sorted_hands = []
    hand_dict = get_card_type(card_list, part)

    for key in hand_dict:
        sorted_hands += [n for (_, n) in insertion_sort(hand_dict[key], part)]

    mults = list(range(1, len(sorted_hands) + 1))[::-1]

    return sum([i * j for i, j in zip(sorted_hands, mults)])

part_one = calculate_winnings(cards, False)
part_two = calculate_winnings(cards, True)

print('--- Day 7: Camel Cards ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')