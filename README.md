# Game of Life: Rust Implementation

Back in 1970 John Horton Conway deviced a game using cellular automata. 

Martin Gardner wrote about the game in his monthly column Mathematical Games in [*Scinetific American*](http://ibiblio.org/lifepatterns/october1970.html).

## Rules

The [rules of the game](http://pi.math.cornell.edu/~lipa/mec/lesson6.html) are quite simple: 

Imagine a graph of paper, devided horizontally and vertically into cells. 

Each of the cells can contain a living creature - a single-cell organism.

With each evolution cycle the decision is made, if the cell lives, dies or is born again.

* If a cell is currently alive but it has fewer than two neighbors, it will die because of lack of support
* If a cell is currently alive and has two or three neighbors, it will survive to the next generation
* If a cell is currently alive and has more than three neighbors, it dies from overpopulation (lack of resources)
* If a cell currently dead but has exaclty three neighbors, it will come back to life