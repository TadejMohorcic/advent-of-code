import random
import copy

with open('2023/Input/2023-25.txt') as input:
    graph = {}

    for line in input:
        node, neighbours = line.strip().split(': ')
        if node not in graph:
            graph[node] = []
        for neighbour in neighbours.split(' '):
            if neighbour not in graph[node]:
                graph[node].append(neighbour)
            if neighbour not in graph:
                graph[neighbour] = [node]
            else:
                if node not in graph[neighbour]:
                    graph[neighbour].append(node)

def random_min_cut(graph):
    initial = {}
    nodes = []
    
    for key in graph:
        initial[key] = (1, graph[key])
        nodes.append(key)

    while len(nodes) > 2:
        n1 = random.choice(nodes)
        found = True
        while found:
            n2 = random.choice(nodes)
            if n2 != n1 and n2 in initial[n1][1]:
                found = False
        
        while n2 in initial[n1][1]:
            initial[n1][1].remove(n2)
        for key in initial:
            while n1 in initial[key][1]:
                initial[key][1].remove(n1)

        for neighbour in initial[n1][1]:
            initial[n2][1].append(neighbour)
            initial[neighbour][1].append(n2)

        initial[n2] = (initial[n2][0] + initial[n1][0], initial[n2][1])
        nodes.remove(n1)
        initial.pop(n1)

    return initial

def size_of_groups(graph):
    
    while True:
        graph_copy = copy.deepcopy(graph)
        result = random_min_cut(graph_copy)
        n1, n2 = list(result.keys())
        if len(result[n1][1]) == 3:
            return result[n1][0] * result[n2][0]

part_one = size_of_groups(graph)

print('--- Day 25: Snowverload ---')
print(f' -  Part one solution: {part_one}.')