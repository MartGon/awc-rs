
# AWC Design

- Entity
    - Types
        - Unit
        - Tile
        - Building
    - Size    

- Components
    - Static 
        - Info
            - Type: Unit/Tile/Building
            - Max HP
            - Size
            - DisplayID
        - Ability (Static)
            - Type
                - Damage
                - Summon
                - Repair
                - Script (Dummy)
                - etc.
            - Pattern
            - Range
        - Effect/Buffs
            - TriggerType (Heartstone keywords)
                - OnDeath
                - OnSummon
                - OnAttack
                - OnMove
                - OnDmgTaken
                - etc.
        - Movement 
            - Types
                - Ground
                - Water
                - Air
            - MaxGas
            - Range
            - MovementPatternID
            - CostTableID
        - Attack
            - Type
                - Projectile
                - Charge/Rush
                - Melee
            - Flag
                - Indirect: Cannot attack after moving.
                - Cannot Attack: This unit cannot attack.
            - Attack pattern
            - Range
        - Build. Controls what entities this unit can build. (Static)
            - Deploy tiles (or Storage)
            - Unit
                - Charges
                - Cost
                - Cooldown
        - Storage. Controls whether this unit can store other units
            - EntityMask

    - Dynamic
        - State
            - HP
        - Transform
            - Position
            - Facing
        - Movement
            - MovementInfo
            - Gas
        - Ability
            - AbilityInfo
            - Charges
            - Cooldown
        - Storage
            - StorageInfo
            - Units stored
        - Build
            - BuildInfo
            - Resources
            - Charges left
        - Attack (Could be replace by Ability)
            - Ammo (Ability: Charges)
        - Effect
            - EffectInfo
            - Charges
            - Duration
            - Dynamic Flags 

- System
    - Command
        - Attack (Implemented as an ability)
            - DoAttack(Attacker, Target)
                - Target
                    - TargetType
                        - Unit
                        - Location
        - Build
        - Move

## Sample units

### Tank

- Type: Unit
- Movement: Moves in a manhatan pattern
- Attack: Can attack units in melee range
- Build: None
- Effects: None
- Storage: None

### Auto Turret

- Type: Building
- Movement: None. It cannot move
- Attack: Attacks units in a cone pattern in front of the turret.
- Build: None
- Effecs:
    - OnMove: Triggers this unit attack. Only affects enemy units.
- Storage: None

### Battlecopter

- Type: Unit
- Movement: Moves in a manhatan pattern. Can go through enemy ground units
- Attack: Can attack in melee range.
- Build: None
- Effects:
    - OnDeath: If possible (wont spawn on water), spawns an infantry unit with half hp
- Storage
    - Infantry

### Air carrier

- Type: Unit
- Movement: Moves in a manhatan pattern.
- Attack: None
- Build:
    - Battlecopter
    - Harrier
- Effects:
- Size:
    2x4x1

## Description

Define an entity by piecing together components.

## Option 1: Every component is an ability

Old components

- Attack -> Basic attack could be implemented as an ability. Similar to wows. Each unit can have its unique basic attack spell
- Build -> Building could be implemented as an specific type of summon. Building has its own set of limitaions, tho.
- Move -> Ability that simply changes position. A teleport ability moves from point a to b. A normal movement moves from point a to b, then from b to c and lastly from c to d.

This helps the trigger/event, cause it only works with abilities. Each ability has its own type which can be listened to.

## Prototype vs Factory

Using the ObjectType pattern, each of the entities has its own type that works as factory. ObjectTypes use static components to describe its type behaviour, however dynamic components are required to store dynamic data (gas, ability cooldowns or number of uses, etc.). This corresponds to using something similar to the Factory pattern

Using a something more similar to a Prototype pattern, each instance can be created by cloning a given template or base info. This way, only dynamic components are needed. The only downside to this is that every unit needs to be loaded beforehand, and space its needed to store their static and dynamic data, even when there are no instances of that unit in use.



