with open('2023/Input/2023-04.txt') as input:
    scratchcards = {}

    for line in input:
        game, numbers = line.strip().split(':')
        game_id = int(game.split()[1])
        winning_numbers, my_numbers = numbers.split('|')
        winning_numbers = [int(n) for n in winning_numbers.split()]
        my_numbers = [int(n) for n in my_numbers.split()]
        scratchcards[game_id] = (winning_numbers, my_numbers)

def calculate_matches(winning_numbers, my_numbers):
    matches = 0

    for number in my_numbers:
        matches += 1 if number in winning_numbers else 0

    return matches

def calculate_points(scratchcard_dict):
    res = 0

    for key in scratchcard_dict:
        winning, mine = scratchcard_dict[key]
        matches = calculate_matches(winning, mine)  
        res += 2 ** (matches - 1) if matches > 0 else 0

    return res

def count_scratchcard(scratchcard_dict):
    number_of_scratchcards = [1 for _ in scratchcard_dict.keys()]

    for key in scratchcard_dict:
        winning, mine = scratchcard_dict[key]
        matches = calculate_matches(winning, mine)
        for i in range(1, matches + 1):
            if key + i <= len(scratchcard_dict):
                number_of_scratchcards[key - 1 + i] += number_of_scratchcards[key - 1]
    
    return sum(number_of_scratchcards)



part_one = calculate_points(scratchcards)
part_two = count_scratchcard(scratchcards)

print('--- Day 4: Scratchcards ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')