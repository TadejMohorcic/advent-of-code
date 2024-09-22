from functools import cache

with open('2015/Input/2015-17.txt') as input:
    containers = set()

    for line in input:
        size = int(line.strip())
        i = 1
        while (size, i) in containers:
            i += 1
        
        containers.add((size, i))

solutions = set()

@cache
def calculate_combinations(litres, container_list):

    if litres == 0:
        if container_list not in solutions:
            solutions.add(container_list)
            return True
        else:
            return False
        
    elif litres < 0:
        return False
    
    else:
        for container in container_list:
            l = list(container_list)
            l.remove(container)
            new_container_list = tuple(l)
            calculate_combinations(litres - container[0], new_container_list)
    
def different_ways(solution_list):
    solution_length = list(sorted([len(x) for x in solution_list]))[::-1]
    last = solution_length[0]
    n = 1

    i = 1
    while True:
        if solution_length[i] == last:
            n += 1
            i += 1
        else:
            return n

calculate_combinations(150, tuple(containers))

part_one = len(solutions)
part_two = different_ways(list(solutions))

print('--- Day 17: No Such Thing as Too Much ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')