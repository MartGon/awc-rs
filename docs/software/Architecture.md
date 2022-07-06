# Game

## Data
- Vec/Map of Entities
- Systems
    - Effect (Event)
    - Attack
    - Move
    - Ability
- Players

## Behaviour
- Actions
    - RunAction(Action) -> Result
- Queries
    - FindEntity(Closure) -> Option(Entity)
    - GetEntityAt(Vec3) -> Option(Entity)

# Entity

- GUID
- Base ID
- EntityType (enum Unit, Tile)
- Transform

# Transform

- Pos (Vec3)
- Orientation (enum N, S, E, W)

# Player

- ID
- TeamId
- Resources

# System<>

## Effect (Event) System


# Components

## Move
 - Range
 - 


# Actions

## Attack - Action

## Move - Action

## Ability - Action

# Database

- EntityTypes
    - Components
        - Attack
        - Movement
        - Abilities
- AbilityTypes
- EffectTypes