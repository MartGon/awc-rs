# TODO
- Draw Unit sprites - DONE
- Make a tileset folder which holds .ron tile info - DONE
- Draw map function - DONE
- Map camera system. - DONE
- Use Log crate - DONE
- Remove factories, use simple HashMaps - DONE
- Use pathfinding crate, implement basic movement - DONE
    - Build Movement Path - DONE
    - Build Movement Area - DONE
        - Show visual effect - DONE

- Serialize/Deserialize template units - DONE
- Implement commands
    - Move - DONE
    - Attack
    - Wait - DONE

- Events
    - Start Turn
    - End Turn
    - Event History on game. Each entry has an ID and turn ID, and triggered by id
    - Commands could return the vec with the triggered events?

- Event Listeners (Component?)
    - The UI should work by using an event listener. Polls every event and plays the corresponding animations.
    - Lua Hooks go here

- Restrict some of the game methods to creation stage
    - Option 1: Don't allow them if the game is started
    - Option 2: Create a pre-game struct with those functions, which create a game struct afterwards: + Less code in game. Call it GamePreview (?)

- Turn System
    - Disabled/Waiting units - DONE
    - HasAttacked units
    - HasMoved units

- Win conditions

- Movement animation
    - Apply movement animation to unit sprite
    - Move sprite around path
        - Flip sprite based on movement direction around path

- MasterFile as trait in an asset crate? / Let the user load everything on their own.
- Check out iced. Looks pretty good

- Display Unit/Terrain info (macroquad UI)

- Player as component? - REJECTED