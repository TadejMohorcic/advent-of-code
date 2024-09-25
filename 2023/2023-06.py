import math

with open('2023/Input/2023-06.txt') as input:
    _, times = input.readline().strip().split(':')
    times = [int(n) for n in times.split()]
    _, distances = input.readline().strip().split(':')
    distances = [int(n) for n in distances.split()]

def join_list(time_list, distance_list):
    time_string, distance_string = '', ''

    for i in range(len(time_list)):
        time_string += str(time_list[i])
        distance_string += str(distance_list[i])

    return int(time_string), int(distance_string)


def number_of_ways(time_list, distance_list):
    res = 1

    for i in range(len(time_list)):
        time, distance = time_list[i], distance_list[i]

        x0, x1 = (time + math.sqrt(time ** 2 - 4 * distance)) / 2, (time - math.sqrt(time ** 2 - 4 * distance)) / 2

        res *= abs(math.ceil(x0) - math.floor(x1) - 1) if x0 > x1 else abs(math.floor(x0) - math.ceil(x1) - 1)

    return res


part_one = number_of_ways(times, distances)
time, distance = join_list(times, distances)
part_two = number_of_ways([time], [distance])

print('--- Day 6: Wait For It ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')