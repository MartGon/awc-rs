use std::collections::HashMap;

use awc::component::EntityID;
use awc::component::EntityType;
use awc::tile;
use awc::{component, component::Component, map};
use awc::game::*;
use awc::player::*;

mod spritesheet;
mod tileset;

use spritesheet::*;
use tileset::BorderMaskEntry;
use tileset::BorderedTile;
use tileset::Borders;
use tileset::BordersMask;

use macroquad::prelude::*;
use macroquad::rand;

use mlua::prelude::*;
use mlua::Table;


#[macroquad::main("BasicShapes")]
async fn main() {

    let lua = Lua::new();

    // Create game
    let mut game = Game::new();
    let pid = game.create_player(TeamID::new(0));

    // Init map
    let water_size = IVec2::new(8, 8);
    for x in 0..water_size.x{
        for y in 0..water_size.y{
            let tile_id = game.create_tile(tile::TypeID::new(0));
            let pos = component::Position{pos : map::Pos{x : x, y : y, z : 0}};
            game.insert_component(tile_id, Component::Position(pos));
        }
    }
    
    let land_anchor = ivec2(2, 2);
    let land_size = IVec2::new(4, 4);
    let corners = vec![ivec2(1, 1), ivec2(1, 6), ivec2(6, 6), ivec2(6, 1)];

    let tiles : Vec<EntityID> = game.map.tiles().cloned().collect();
    for tile in tiles{ 
        let pos = game.components_mut().get_position(&tile).unwrap();
        let pos = ivec2(pos.pos.x, pos.pos.y);
        if pos.x < land_size.x + land_anchor.x && pos.y < land_size.y + land_anchor.y &&
            pos.x >= land_anchor.x && pos.y >= land_anchor.y ||
            corners.contains(&pos)
        {
            game.components_mut().get_type_mut(&tile).unwrap().entity_type = EntityType::Tile(tile::TypeID::new(rand::gen_range(1, 3)));
        }

    }

    // Load SpriteSheet
    let spritesheet = Image::from_file_with_format(include_bytes!("../../sprites/spritesheet2.png"), Some(ImageFormat::Png));
    let spritesheet = Texture2D::from_image(&spritesheet);

    // Tile Sprites
    let tile_size = Vec2::new(16.0, 16.0);
    let grass = BorderedTile::new_short(sprite_raw(3, 14, tile_size.x as i32, tile_size.y as i32));
    let mountain = BorderedTile::new_short(sprite_raw(71, 14, tile_size.x as i32, tile_size.y as i32));
    let water = BorderedTile::new(  
        animated_sprite(tile_size.as_uvec2(), &[
            Animation::new_shorter_y("idle".to_string(), 4, 20, &[
                129, 199, 269, 339, 339, 269, 129
            ])
        ]),
        &[


            // ***** Surrounded by 3 ****** \\
            // Top-Left-Right
            (BordersMask::new_short(BorderMaskEntry::some(&[1, 2]), &[ivec2(0, -1), ivec2(1, 0), ivec2(-1, 0)]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 54, &[
                    112, 182, 252, 322, 322, 252, 182, 112,
                ]),
            ])),
            // Left-Right-Bottom
            (BordersMask::new_short(BorderMaskEntry::some(&[1, 2]), &[ivec2(0, 1), ivec2(-1, 0), ivec2(1, 0)]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 54, &[
                    146, 216, 286, 356, 356, 286, 216, 146,
                ]),
            ])),
            // Bottom-Left-Top
            (BordersMask::new_short(BorderMaskEntry::some(&[1, 2]), &[ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 3, &[
                    163, 233, 303, 373, 373, 303, 233, 163,
                ]),
            ])),
            // Bottom-Right-Top
            (BordersMask::new_short(BorderMaskEntry::some(&[1, 2]), &[ivec2(0, 1), ivec2(1, 0), ivec2(0, -1)]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 37, &[
                    163, 233, 303, 373, 373, 303, 233, 163,
                ]),
            ])),
            
            // ***** Corners ******* \\
            // Top-Right
            (BordersMask::new_short(BorderMaskEntry::some(&[1, 2]), &[ivec2(0, -1), ivec2(1, 0)]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 37, &[
                    112, 182, 252, 322, 322, 252, 182, 112,
                ]),
            ])),
            // Top-Left
            (BordersMask::new_short(BorderMaskEntry::some(&[1, 2]), &[ivec2(0, -1), ivec2(-1, 0)]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 3, &[
                    112, 182, 252, 322, 322, 252, 182, 112,
                ]),
            ])),
            // Bottom-Right
            (BordersMask::new_short(BorderMaskEntry::some(&[1, 2]), &[ivec2(0, 1), ivec2(1, 0)]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 37, &[
                    146, 216, 286, 356, 356, 286, 216, 146,
                ]),
            ])),
            // Bottom-Left
            (BordersMask::new_short(BorderMaskEntry::some(&[1, 2]), &[ivec2(0, 1), ivec2(-1, 0)]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 3, &[
                    146, 216, 286, 356, 356, 286, 216, 146,
                ]),
            ])),

            // ***** Single Sides ****** \\
            // Grass on Top
            (BordersMask::new(&[(ivec2(0, -1), BorderMaskEntry::some(&[1, 2]))]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 20, &[
                    112, 182, 252, 322, 322, 252, 182, 112,
                ]),
            ])),
            // Grass on Bottom
            (BordersMask::new(&[(ivec2(0, 1), BorderMaskEntry::some(&[1, 2]))]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 20, &[
                    146, 216, 286, 356, 356, 286, 216, 146,
                ]),
            ])),
            // Right
            (BordersMask::new(&[(ivec2(1, 0), BorderMaskEntry::some(&[1, 2]))]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 37, &[
                    129, 199, 269, 339, 339, 269, 129
                ]),
            ])),
            // Left
            (BordersMask::new(&[(ivec2(-1, 0), BorderMaskEntry::some(&[1, 2]))]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 3, &[
                    129, 199, 269, 339, 339, 269, 129
                ]),
            ])),

            // ***** Dot Corners ******* \\
            // Right-Top Corner
            (BordersMask::new(&[(ivec2(1, -1), BorderMaskEntry::some(&[1, 2]))]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 71, &[
                    129, 199, 269, 339, 339, 269, 129
                ]),
            ])),
            // Left-Top Corner
            (BordersMask::new(&[(ivec2(-1, -1), BorderMaskEntry::some(&[1, 2]))]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 88, &[
                    129, 199, 269, 339, 339, 269, 129
                ]),
            ])),
            // Right-Bottom Corner
            (BordersMask::new(&[(ivec2(1, 1), BorderMaskEntry::some(&[1, 2]))]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 71, &[
                    112, 182, 252, 322, 322, 252, 182, 112,
                ]),
            ])),
            // Left-Bottom Corner
            (BordersMask::new(&[(ivec2(-1, 1), BorderMaskEntry::some(&[1, 2]))]),  animated_sprite(tile_size.as_uvec2(), &[
                Animation::new_shorter_y("idle".to_string(), 4, 88, &[
                    112, 182, 252, 322, 322, 252, 182, 112,
                ]),
            ])),
        ]);
    
    let mut tileset : HashMap<tile::TypeID, BorderedTile> = HashMap::new();
    tileset.insert(tile::TypeID::new(0), water);
    tileset.insert(tile::TypeID::new(1), grass);
    tileset.insert(tile::TypeID::new(2), mountain);

    let mut tile_type = tile::TypeID::new(0);
    loop {

        let scale = vec2(2.0, 2.0);
        let draw_size = tile_size * scale;
        // Inpput handling \\
        let (x, y) = mouse_position();
        let tile_pos = (vec2(x, y) / draw_size).as_ivec2();
        let tile_pos = awc::map::Pos::new(tile_pos.x, tile_pos.y, 0);

        if is_mouse_button_released(MouseButton::Left){
            if let Some(tile) = game.get_tile_in_pos(&tile_pos){
                game.components_mut().get_type_mut(&tile).unwrap().entity_type = EntityType::Tile(tile_type);
            }
        }

        if is_mouse_button_released(MouseButton::Right){
            if let Some(tile) = game.get_tile_in_pos(&tile_pos){
                if let EntityType::Tile(id) = game.components_mut().get_type_mut(&tile).unwrap().entity_type{
                    tile_type = id;
                }
            }
        }

        // Drawing \\
        clear_background(RED);

        for tile in game.map.tiles(){            
            let tile_pos = &game.components().get_position(tile).unwrap().pos;
            //let tile_pos = awc::map::Pos::new(1, 2, 0);
            let draw_pos = Vec2::new(tile_pos.x as f32 * draw_size.x, tile_pos.y as f32 * draw_size.y);

            let ttype = game.components().get_type(tile).unwrap();
            if let EntityType::Tile(ttype) = ttype.entity_type {
                if let Some(tile_sprite) = tileset.get_mut(&ttype){

                    // Calculate borders
                    let mut borders = Borders::default();
                    for x in tileset::OFFSET_MIN..tileset::OFFSET_MAX + 1{
                        for y in tileset::OFFSET_MIN..tileset::OFFSET_MAX + 1{
                            let offset = ivec2(x, y);
                            let pos = map::Pos::new(tile_pos.x + x, tile_pos.y + y, 0);

                            if let Some(up) = game.get_tile_in_pos(&pos){
                                let ttype = game.components().get_type(&up).unwrap();
                                if let EntityType::Tile(ttype) = ttype.entity_type{
                                    borders.insert(offset, ttype);
                                }
                            }
                        }
                    }
                    
                    // Draw tile
                    let sprite = tile_sprite.sprite(&borders);
                    match sprite{
                        SpriteType::Sprite(s) => s.draw_scaled(&spritesheet, draw_pos, scale),
                        SpriteType::AnimatedSprite(s) => s.draw_scaled(&spritesheet, draw_pos, scale)
                    }
                }
            }
        }
        
        /*
        // Draw UI
        if root_ui().window(1, Vec2::new(screen_width() / 2.0, screen_height() / 2.0), Vec2::new(86.0, 64.0), |ui|{
            if ui.button(None, "Push me") {
                println!("pushed");
            }}){

        }
         */

        next_frame().await
    }
}