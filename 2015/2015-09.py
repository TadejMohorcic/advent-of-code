import heapq

with open('2015/Input/2015-09.txt') as input:
    paths = []
    cities = set()

    for line in input:
        c = line.strip().split()
        paths.append((c[0], c[2], int(c[4])))
        cities.add(c[0])
        cities.add(c[2])

def create_graph(edges):
    graph = {}

    for e in edges:
        city_1, city_2, cost = e[0], e[1], e[2]

        try:
            graph[city_1][city_2] = cost
        except KeyError:
            graph[city_1] = {}
            graph[city_1][city_2] = cost

        try:
            graph[city_2][city_1] = cost
        except KeyError:
            graph[city_2] = {}
            graph[city_2][city_1] = cost

    return graph

def calculate_cost(graph):
    q = []
    seen = []

    for c in cities:
        q.append((0, [c]))

    while q != []:
        cost, path = heapq.heappop(q)
        prev_city = path[-1]

        for c in cities:
            if c not in path:
                new_cost = cost + graph[prev_city][c]
                new_path = path.copy()
                new_path.append(c)
                if len(new_path) == len(cities):
                    heapq.heappush(seen, (new_cost, new_path))
                else:
                    heapq.heappush(q, (new_cost, new_path))

    return seen

def path_length(path_list, part):
    graph = create_graph(path_list)
    visited = calculate_cost(graph)

    if part:
        while len(visited) != 1:
            heapq.heappop(visited)

    return heapq.heappop(visited)[0]
        
part_one = path_length(paths, False)
part_two = path_length(paths, True)

print('--- Day 9: All in a Single Night ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')