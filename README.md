# Graph coloring solver with simulated annealing

This tiny library (toy project) solves the [graph coloring
problem](https://en.wikipedia.org/wiki/Graph_coloring) by [simulated
annealing](https://en.wikipedia.org/wiki/Simulated_annealing).

The provided binary reads the description of a graph on a file or the
standard input, with the following format:

* `N`, `E` on the first line (number of nodes, number of edges)
* `a_i`, `b_i` on the `E` next lines (where `a_i` and `b_i` are nodes of the
    graph linked by an edge).

Then, it attempts to solve the graph coloring problem with decreasing
numbers of colors, and write the solutions to files.
