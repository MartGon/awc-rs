# Design Ideas

## Entity - Component - System

Everything is an entity. Entities have different types.

- Entities: Units, Tiles
- Components: Transform, Attack, Movement, Effect, Ability
- System: Attack, Movement, Effect/Event

**Option 2**: Component only model. Check zemeroth for inspiration. 

### Event System

The effect component handles different events. 

Example Event types: UNIT_MOVE, UNIT_TAKE_DMG, UNIT_DIE, UNIT_ATTACK...

Use a stack to resolve events, similar to Magic's

## Examples

The engine must support the implementation of the following entities

### Units

- **Tank**: Has a Transform, Attack and Movement component.
- **Artillery**: Has a Tranform, Attack and Ability component
    - **Aim (Ability)**: Applies a buff (effect component). Next attack has more range and precision.
- **Sentry Turret**: Has a Transform, Effect, Attack component
    - **Sentry**: Attacks an enemy unit that moves into its attack range. It cannot attack.

### Tiles

- **Cracked Floor**: Has a Transform and an Effect component
    - **Crack**: The tile cracks after an unit leaves. Effectively disappearing
- **Wildfire**: Has a Transform and an Effect component.
    - **Fire**: Units on this tile take periodic fire damage.


## Random ideas

- **Speed**: Units can take actions sooner based on their speed. Think of Final Fantasy Tactics. This gets rid of the idea of traditional turns. Can lead to more frequent interaction between players in multiplayer

## Must have features

- **Pre-Move**: During your opponent's turn, you may move your units, once it's complete, invalid actions are rejected. Must be part of the game, not the core.
- **Scripting Support**: Unit effects and/or actions can be implemented in a scripting language, such as Lua.

## Undoable Actions vs State Diff

- **Undoable Actions** have to extend to triggered effects as well in order to work properly.
- **State Diff**: After an action is taken, and all effects are resolved, a diff is calculated. In order to undo, this diff is applied to the current state.

## Entity Manager vs Systems

Components are stored in the entity manager, instead of systems. Easy of access can be provided by this class, instead of by each. Could be done directly by game, I dunno.

## Object Type vs Cloning

Something to consider. Object Type should reduce memory usage but Cloning is way simpler.

## Attacking units vs Attacking tiles

Attacking units forces the attack system to know if a given unit can a attack each one of the existing unit types. However, by using an attack table which holds attackable tile types, the number of entries can be reduced drastically. For instance, a submarine should never be able to attack with its torpedos an air tile or ground tile.

## Logs and Decoupling

After every action taken by the game, a log should be emitted describring each of the events that were triggered because of that action. This could be use for multiplayer games, as this information would be generated by the game server and sent to the clients.

Similarly, this info could be used by the game engine to show the visual effects related with those actions.

## Designing upfront is hard

Trying to anticipate to every problem that may occur is impossible. Constant iteration will be mandatory, so it should be better to start with some foundations that are most probably needed. Those are the components and their dependencies, such as the AreaGenerator or AreaDescriptor

## Make the base game

Create the base game. Then, add modifications or unit traits with the extra behaviour. Change the code accordingly.
E.g. Instead of allowing every unit to move after attacking, make it a unit trait/perk. Make the necessary modifications.

## Crates to use 

rlua
pathfinding
ron
miniquad
egui
egui-miniquad


## Crates to check

rs-tiled
aseprite
macroquad
mlua
image
egui
glium
miniquad
macroquad