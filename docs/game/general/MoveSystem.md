# Battle System

## Tile cost

Move to a tile has a specific cost associated with it. This cost depends on the unit moving. We have two options here:

- **The unit has move cost table**: This is simple, but may be prone to repetition if many units have the same movement consts
- **The unit has move type, which has the cost table**: A bit more complex, but could reduce repetition and save memory.

Due to the fact that the map is 3D, **climbing** to another block can have an associated extra cost, which is added to the destination tile.

##  Movement patterns

Various ways to define them:

- Manhattan: Moves in a cross pattern
- Regex: Movement must match a regular expression.
- Function: Move up and left the same number of tiles


## Ideas

- Units that move by jumping.
- Keep it simple. Duplication is not a real probelm. Memory is exendable nowadays