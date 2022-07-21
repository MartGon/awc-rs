# Map Desgin

- Isometric view

## Idea 1: Forest and mountains as units

Water, Air and Land blocks. They make all the map

Mountains, Forests (or Trees) and buildings are units that stay on top of land blocks. They can be destroyed.

Road blocks are similar to land block, but have a different top sprite. They also enhance movement of land units.

## Idea 2: Destructible terrain

Destructible terrain bring a bunchs of problems or limitaions. It affects gameplay and makes sense visually if a bomb makes a crater. However, this pretty much forces the use of 3d models. This it due to the fact that, for instance, a tunnel won't never be able to be seen completely by using static camera angles and isometric 2d art.

Using a 3d models and camera, pretty much fixes this problem.

However, despite complex, partial solution can be used to solve this problem while sticking to an isometric 2d view. It'd have to be a combination of optionally hiding terrain and detecting when a given block is hidden out of the camera view by other block. Tycoon games had this problems in some degree.

**Tiles are Units**

In this model, tiles could behave exactly like units. They've an HP bar that shows how close they're to being destroyed. For movement, units would only have a cost table that holds only units.