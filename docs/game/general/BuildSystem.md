# Build System

## Implementation Options

Different approaches exist:

1. Build as abilities. Building a specific unit is a unique ability. It can be used by different units, such as factories or engineers

2. Engineer Effect. Units with this effect can access the build system. The units they can build are taken from a table.

3. Every unit can build. Similar to Option 2, but there's only a table. Virtually every unit has the Engineer effect.

## Where the built unit should be placed?

Two main options:

1. Before the build process starts, a tile (or set of tiles) needs to be selected, where the unit will be built. Factories, can choose a tile where the unit to be built will spawn (Somewhat similar to Wargroove). Units cannot hold or carry other units in a virtual space, they always have to be seen.

2. Cargo system (similar to AW). Units built by other units are held on their cargo, and they cannot be directly targeted. Factories build units on top of them.

Option 1 is generally preferred

## Extra notes

**Build Range**: Different units have different build range. Higher level engineers can build other units from afar.

**Multiple workers**:  Once a unit build process start, another engineer can help the build process, reducing the number of turns needed to build that unit.s

## Example factory

It's a unit like any other, but it cannot move. It has size 3x1x3. A set of tiles is given where units can be built. It cannot attack, it cannot move. It can create more than one unit per turn, until it fills all available build tiles

