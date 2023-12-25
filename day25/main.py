from collections import defaultdict

import networkx as nx


# Day 25 using networkx, this Python Complex Graph Library is just too op ...
def main():
    graph = defaultdict(list)

    with open("input.txt") as f:
        file = f.readlines()
        for line in file:
            source, destination = line.strip().rsplit(':', maxsplit=1)

            for d in filter(lambda x: x, destination.split(' ')):
                graph[source].append(d)

    G = nx.from_dict_of_lists(graph)
    min_cut = nx.minimum_edge_cut(G)
    G.remove_edges_from(min_cut)
    c1, c2 = nx.connected_components(G)
    print(len(c1) * len(c2))


if __name__ == "__main__":
    main()
