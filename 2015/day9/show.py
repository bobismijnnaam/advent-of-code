with open('input') as f:
    lines = f.readlines()

lines = [x.split("=")[0] for x in lines]
lines = [x.split("to") for x in lines]
edges = [(x[0].strip(), x[1].strip()) for x in lines]

import networkx as nx
import matplotlib.pyplot as plt

G = nx.Graph(edges)

nx.draw(G)
plt.draw()
plt.show()
