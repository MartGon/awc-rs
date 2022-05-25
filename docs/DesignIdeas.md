# Design Ideas

## Entity - Component - System

Everything is an entity. Entities have different types.

- Entities: Units, Tiles
- Components: Transform, Attack, Movement, Effect, Ability
- System: Attack, Movement, Effect/Event

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
