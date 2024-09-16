with open('2016/Input/2016-20.txt') as input:
    ranges = []
    for line in input:
        start, end = line.strip().split('-')
        ranges.append((int(start), int(end)))

def get_non_valid(range_list):
    non_valid = []

    for r in range_list:
        start, end = r
        to_remove = []

        for r2 in non_valid:
            start_2, end_2 = r2
            if start > end_2 or end < start_2:
                continue
            else:
                to_remove.append(r2)
                start = min(start, start_2)
                end = max(end, end_2)

        for r2 in to_remove:
            non_valid.remove(r2)

        non_valid.append((start, end))
    
    return sorted(non_valid)

def valid_ips(range_list, part):
    non_valid = get_non_valid(range_list)
    valid = 0

    for i in range(len(non_valid) - 1):
        end = non_valid[i][1]
        next_start = non_valid[i + 1][0]

        if part:
            valid += (next_start - end - 1)
        else:
            if end + 1 != next_start:
                valid = end + 1
                break
            
    return valid

part_one = valid_ips(ranges, False)
part_two = valid_ips(ranges, True)

print('--- Day 20: Firewall Rules ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')