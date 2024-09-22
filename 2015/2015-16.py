ticker_tape = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1]

with open('2015/Input/2015-16.txt') as input:
    text_list = []
    for line in input:
        text = line.strip().split()
        for i in range(len(text)):
            if i % 2 != 0:
                if i != len(text) - 1:
                    text[i] = text[i][:-1]
        text_list.append(text)

def extract_data(data_list):
    extracted_data = []

    for d in data_list:
        n = len(d)
        i = 2
        data = [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1]
        index = int(d[1])

        while i < n:
            match d[i]:
                case 'children:':
                    data[0] = int(d[i+1])
                case 'cats:':
                    data[1] = int(d[i+1])
                case 'samoyeds:':
                    data[2] = int(d[i+1])
                case 'pomeranians:':
                    data[3] = int(d[i+1])
                case 'akitas:':
                    data[4] = int(d[i+1])
                case 'vizslas:':
                    data[5] = int(d[i+1])
                case 'goldfish:':
                    data[6] = int(d[i+1])
                case 'trees:':
                    data[7] = int(d[i+1])
                case 'cars:':
                    data[8] = int(d[i+1])
                case 'perfumes:':
                    data[9] = int(d[i+1])
            i += 2

        extracted_data.append((index, data))
    return extracted_data

def check_data(data_list, part):
    
    for d in data_list:
        index, data = d
        correct_sue = True

        for i in range(len(ticker_tape)):
            if data[i] == -1:
                continue
            else:
                if part:
                    if i in [1, 7]:
                        if data[i] <= ticker_tape[i]:
                            correct_sue = False
                    elif i in [3, 6]:
                        if data[i] >= ticker_tape[i]:
                            correct_sue = False
                    else:
                        if data[i] != ticker_tape[i]:
                            correct_sue = False
                else:
                    if data[i] != ticker_tape[i]:
                        correct_sue = False

        if correct_sue:
            return index
        
part_one = check_data(extract_data(text_list), False)
part_two = check_data(extract_data(text_list), True)

print('--- Day 16: Aunt Sue ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')