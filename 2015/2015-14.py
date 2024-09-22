with open('2015/Input/2015-14.txt') as input:
    reindeers = []
    reindeers_data = []

    for line in input:
        text = line.strip().split()
        reindeer = int(text[3]), int(text[6]), int(text[-2])
        reindeers.append(reindeer)
        reindeers_data.append([int(text[3]), int(text[6]), int(text[-2]), 0, 0, int(text[6]), True])

def calculate_distance(reindeer_list, time):
    max_distance = 0
    
    for reindeer in reindeer_list:
        v, t1, t2 = reindeer
        total_time = t1 + t2
        distance = t1 * v

        time_left = time % total_time - t1 if time % total_time - t1 > 0 else 0
        time_left = time_left if time_left <= t1 else t1

        n = time // total_time

        distance = n * distance + time_left * v
        
        if distance > max_distance:
            max_distance = distance

    return max_distance

def score_points(reindeer_data, time):

    for i in range(time):
        for j in range(len(reindeer_data)):
            if reindeer_data[j][-1] == True:
                if reindeer_data[j][-2] > 0:
                    reindeer_data[j][-2] -= 1
                    reindeer_data[j][-3] += reindeer_data[j][0]
                else:
                    reindeer_data[j][-2] = reindeer_data[j][2] - 1
                    reindeer_data[j][-1] = False
            else:
                if reindeer_data[j][-2] > 0:
                    reindeer_data[j][-2] -= 1
                else:
                    reindeer_data[j][-2] = reindeer_data[j][1] - 1
                    reindeer_data[j][-3] += reindeer_data[j][0]
                    reindeer_data[j][-1] = True

        index, dist = [], 0
        for j in range(len(reindeer_data)):
            if reindeer_data[j][-3] > dist:
                dist = reindeer_data[j][-3]
                index = [j]
            elif reindeer_data[j][-3] == dist:
                index.append(j)
        
        for j in index:
            reindeer_data[j][-4] += 1

    points = [reindeer_data[i][3] for i in range(len(reindeer_data))]

    return max(points)

part_one = calculate_distance(reindeers, 2503)
part_two = score_points(reindeers_data, 2503)

print('--- Day 14: Reindeer Olympics ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')