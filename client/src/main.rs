use awc::component::EntityID;
use awc::component::EntityType;
use awc::tile;
use macroquad::prelude::*;
use macroquad::rand;
use macroquad::ui::*;
use awc::{component, component::Component, map};
use awc::game::*;
use awc::player::*;

mod spritesheet;
mod tileset;

use spritesheet::*;
use tileset::Border;
use tileset::BorderedTile;
use tileset::Borders;


#[macroquad::main("BasicShapes")]
async fn main() {

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

    let land_size = IVec2::new(4, 4);
    let tiles : Vec<EntityID> = game.map.tiles().cloned().collect();
    for tile in tiles{ 
            let pos = game.components_mut().get_position(&tile).unwrap();
            if pos.pos.x < land_size.y && pos.pos.y < land_size.y{
                game.components_mut().get_type_mut(&tile).unwrap().entity_type = EntityType::Tile(tile::TypeID::new(rand::gen_range(1, 2)));
            }
        }

    // Load SpriteSheet
    let spritesheet = Image::from_file_with_format(include_bytes!("../../sprites/spritesheet.png"), Some(ImageFormat::Png));
    let spritesheet = Texture2D::from_image(&spritesheet);

    // Tile Sprites
    let tile_size = Vec2::new(16.0, 16.0);
    let mut grass = Sprite::new_raw(238, 18, tile_size.x as i32, tile_size.y as i32);
    let mut mountain = Sprite::new_raw(255, 18, tile_size.x as i32, tile_size.y as i32);
    let mut water = BorderedTile::new(  
        AnimatedSprite::new(tile_size.as_u32(), &[
            Animation::new("idle".to_string(), 4, &[
                AnimationFrame::new(ivec2(12, 133)),
                AnimationFrame::new(ivec2(31, 133)),
                AnimationFrame::new(ivec2(48, 133)),
                AnimationFrame::new(ivec2(65, 133)),
                AnimationFrame::new(ivec2(65, 133)),
                AnimationFrame::new(ivec2(48, 133)),
                AnimationFrame::new(ivec2(31, 133)),
                AnimationFrame::new(ivec2(12, 133)),
            ])
        ]),
        &[
            (Borders{top: Border::Some(tile::TypeID::new(1)), ..Default::default()},  AnimatedSprite::new(tile_size.as_u32(), &[
                Animation::new("idle".to_string(), 4, &[
                    AnimationFrame::new(ivec2(12, 150)),
                    AnimationFrame::new(ivec2(31, 150)),
                    AnimationFrame::new(ivec2(48, 150)),
                    AnimationFrame::new(ivec2(65, 150)),
                    AnimationFrame::new(ivec2(65, 150)),
                    AnimationFrame::new(ivec2(48, 150)),
                    AnimationFrame::new(ivec2(31, 150)),
                    AnimationFrame::new(ivec2(12, 150)),
                ]),
            ])),
            (Borders{left : Border::Some(tile::TypeID::new(1)), ..Default::default()},  AnimatedSprite::new(tile_size.as_u32(), &[
                Animation::new("idle".to_string(), 4, &[
                    AnimationFrame::new(ivec2(12, 218)),
                    AnimationFrame::new(ivec2(31, 218)),
                    AnimationFrame::new(ivec2(48, 218)),
                    AnimationFrame::new(ivec2(65, 218)),
                    AnimationFrame::new(ivec2(65, 218)),
                    AnimationFrame::new(ivec2(48, 218)),
                    AnimationFrame::new(ivec2(31, 218)),
                    AnimationFrame::new(ivec2(12, 218)),
                ]),
            ])),
            (Borders{top_left : Border::Some(tile::TypeID::new(1)),  ..Default::default()},  AnimatedSprite::new(tile_size.as_u32(), 
            &[
                Animation::new("idle".to_string(), 4, &[
                    AnimationFrame::new(ivec2(12, 286)),
                    AnimationFrame::new(ivec2(31, 286)),
                    AnimationFrame::new(ivec2(48, 286)),
                    AnimationFrame::new(ivec2(65, 286)),
                    AnimationFrame::new(ivec2(65, 286)),
                    AnimationFrame::new(ivec2(48, 286)),
                    AnimationFrame::new(ivec2(31, 286)),
                    AnimationFrame::new(ivec2(12, 286)),
                ]),
            ])),
        ]);
    
    loop {
        clear_background(RED);

        let scale = vec2(2.0, 2.0);
        let draw_size = tile_size * scale;

        for tile in game.map.tiles(){            
            let tile_pos = &game.components().get_position(tile).unwrap().pos;
            let draw_pos = Vec2::new(tile_pos.x as f32 * draw_size.x, tile_pos.y as f32 * draw_size.y);

            let ttype = game.components().get_type(tile).unwrap();
            if let EntityType::Tile(ttype) = ttype.entity_type {
                match ttype.0 {
                    0 => {
                        let mut borders = Borders::default();
                        let up_pos = map::Pos::new(tile_pos.x, tile_pos.y - 1, 0);
                        if let Some(up) = game.get_tile_in_pos(up_pos){
                            let ttype = game.components().get_type(&up).unwrap();
                            if let EntityType::Tile(ttype) = ttype.entity_type{
                                borders.top = Border::Some(ttype);
                            }
                        }

                        let left_pos = map::Pos::new(tile_pos.x - 1, tile_pos.y , 0);
                        if let Some(up) = game.get_tile_in_pos(left_pos){
                            let ttype = game.components().get_type(&up).unwrap();
                            if let EntityType::Tile(ttype) = ttype.entity_type{
                                borders.left = Border::Some(ttype);
                            }
                        }

                        let top_left_pos = map::Pos::new(tile_pos.x - 1, tile_pos.y - 1 , 0);
                        if let Some(up) = game.get_tile_in_pos(top_left_pos){
                            let ttype = game.components().get_type(&up).unwrap();
                            if let EntityType::Tile(ttype) = ttype.entity_type{
                                borders.top_left = Border::Some(ttype);
                            }
                        }
                        
                        println!("Pos: {:?}", tile_pos);
                        println!("Borders: {:?}", borders);
                        let sprite = water.sprite(&borders);
                        sprite.draw_scaled(&spritesheet, draw_pos, scale);
                    },
                    1 => grass.draw_scaled(&spritesheet, draw_pos, scale),
                    2 => mountain.draw_scaled(&spritesheet, draw_pos, scale),
                    _ => {},
                };
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