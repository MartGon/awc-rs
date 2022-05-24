# Design Ideas

## Entity - Component - System

Everything is an entity. Entities have different types.

- Entities: Units, Tiles
- Components: Transform, Attack, Movement, Effect, Ability
- System: Attack, Movement, Effect/Event

### Event System

The effect component handles different events. 

Example Event types: UNIT_MOVE, UNIT_TAKE_DMG, UNIT_DIE, UNIT_ATTACK...

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
