with open('2023/Input/2023-05.txt') as input:
    _, seeds = input.readline().strip().split(':')
    seeds = [int(n) for n in seeds.split()]
    input.readline()

    maps_dict = {}
    current_map = []

    for line in input:
        if line != '\n':
            current_map.append(line.strip())
        else:
            map_name = current_map[0].split()[0]
            maps_dict[map_name] = []
            for r in current_map[1:]:
                maps_dict[map_name].append([int(n) for n in r.split()])
            current_map = []
    
    map_name = current_map[0].split()[0]
    maps_dict[map_name] = []
    for r in current_map[1:]:
        maps_dict[map_name].append([int(n) for n in r.split()])

def seed_to_range(seed_list):
    seed_ranges = []

    for i in range (0, len(seeds), 2):
        seed_ranges.append((seeds[i], seeds[i] + seeds[i + 1]))

    return seed_ranges

def update_seeds(seed_list, map_dict):
    updated_seeds = {s: s for s in seeds}

    for key in map_dict:
        for seed in seed_list:
            for destination, source, range_len in map_dict[key]:
                if source <= updated_seeds[seed] <= source + range_len - 1:
                    updated_seeds[seed] = updated_seeds[seed] + destination - source
                    break

    return min([updated_seeds[x] for x in seeds])

def update_intervals(range_list, map_dict):
    
    for key in map_dict:
        updated_ranges = []

        while len(range_list) > 0:
            start, end = range_list.pop()

            for destination, source, range_len in map_dict[key]:
                overlap_start = max(start, source)
                overlap_end = min(end, source + range_len)

                if overlap_start < overlap_end:
                    updated_ranges.append((overlap_start + destination - source, overlap_end + destination - source))
                    if overlap_start > start:
                        range_list.append((start, overlap_start))
                    if end > overlap_end:
                        range_list.append((overlap_end, end))
                    break
            else:
                updated_ranges.append((start, end))

        range_list = updated_ranges

    return min([n[0] for n in range_list])

part_one = update_seeds(seeds, maps_dict)
part_two = update_intervals(seed_to_range(seeds), maps_dict)

print('--- Day 5: If You Give A Seed A Fertilizer ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')