import heapq

with open('2015/Input/2015-13.txt') as input:
    seating = {}

    for line in input:
        txt = line.strip().split()
        p1, p2 = txt[0], txt[-1][:-1]
        n = int(txt[3])
        
        if txt[2] == 'lose':
            num = -1 * n
        else:
            num = n
        
        if p1 not in seating:
            seating[p1] = {}
        seating[p1][p2] = num

def create_graph(seat_list):
    graph = {}

    for p in seat_list:
        sub_g = {}
        for q in seat_list[p]:
            cost = seat_list[p][q]
            if q in graph:
                s = cost + graph[q][p]
                sub_g[q] = s
                graph[q][p] = s
            else:
                sub_g[q] = cost

        graph[p] = sub_g
    
    return graph

def add_me(graph):
    g = graph.copy()
    g['Me'] = {}
    for p in g:
        if p != 'Me':
            g[p]['Me'] = 0
            g['Me'][p] = 0

    return g

def calculate_happiness(graph):
    queue = []
    max_happiness = 0

    person = list(graph.keys())[0]
    queue.append((0, [person]))

    while queue != []:
        happiness, seat_order = heapq.heappop(queue)
        prev_person = seat_order[-1]

        for next_person in graph:
            if next_person not in seat_order:
                new_seat_order = seat_order.copy()
                new_seat_order.append(next_person)
                new_happiness = happiness + graph[prev_person][next_person]
                if len(new_seat_order) == len(graph):
                    updated_happiness = new_happiness + graph[person][next_person]
                    if updated_happiness > max_happiness:
                        max_happiness = updated_happiness
                else:
                    heapq.heappush(queue, (new_happiness, new_seat_order))

    return max_happiness

graph = create_graph(seating)
part_one = calculate_happiness(graph)

graph_2 = add_me(graph)
part_two = calculate_happiness(graph_2)

print('--- Day 13: Knights of the Dinner Table ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')