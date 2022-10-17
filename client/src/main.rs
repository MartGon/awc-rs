use std::fs;

use awc::component::EntityID;
use awc::component::EntityType;
use awc::tile;
use awc::{component, component::Component, map};
use awc::game::*;
use awc::player::*;

mod spritesheet;
mod tileset;

use spritesheet::*;

use macroquad::prelude::*;
use tileset::Borders;


#[macroquad::main("BasicShapes")]
async fn main() {

    // Create game
    let mut game = Game::new();
    let pid = game.create_player(TeamID::new(0));

    // Init map
    let water_size = UVec2::new(8, 8);
    for x in 0..water_size.x{
        for y in 0..water_size.y{
            let tile_id = game.create_tile(tile::TypeID::new(0));
            let pos = component::Position{pos: uvec2(x, y)};
            game.insert_component(tile_id, Component::Position(pos));
        }
    }
    
    let land_anchor = uvec2(2, 2);
    let land_size = UVec2::new(4, 4);
    let corners = vec![uvec2(1, 1), uvec2(1, 6), uvec2(6, 6), uvec2(6, 1)];

    let tiles : Vec<EntityID> = game.map.tiles().cloned().collect();
    for tile in tiles{ 
        let pos = game.components_mut().get_position(&tile).unwrap();
        let pos = uvec2(pos.pos.x, pos.pos.y);
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
    
    // Load tileset 
    let tileset_str = fs::read_to_string("sprites/tileset.ron").expect("Error while reading tileset info file");
    let tileset = ron::from_str::<tileset::Tileset>(&tileset_str).expect("Error on deserialize tileset");
    let tile_size = Vec2::new(16.0, 16.0);

    let mut tile_type = tile::TypeID::new(0);
    loop {

        let scale = vec2(2.0, 2.0);
        let draw_size = tile_size * scale;
        // Inpput handling \\
        let (x, y) = mouse_position();
        let tile_pos = (vec2(x, y) / draw_size).as_uvec2();

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
            let tile_pos = &game.components().get_position(tile).unwrap().pos.as_ivec2();
            //let tile_pos = awc::map::Pos::new(1, 2, 0);
            let draw_pos = Vec2::new(tile_pos.x as f32 * draw_size.x, tile_pos.y as f32 * draw_size.y);

            let ttype = game.components().get_type(tile).unwrap();
            if let EntityType::Tile(ttype) = ttype.entity_type {
                if let Some(tile_sprite) = tileset.get(&ttype){

                    // Calculate borders
                    let mut borders = Borders::default();
                    for x in tileset::OFFSET_MIN..tileset::OFFSET_MAX + 1{
                        for y in tileset::OFFSET_MIN..tileset::OFFSET_MAX + 1{
                            let offset = ivec2(x, y);
                            let pos = *tile_pos + offset;
                            if pos.x >= 0 && pos.y > 0 {
                                if let Some(up) = game.get_tile_in_pos(&pos.as_uvec2()){
                                    let ttype = game.components().get_type(&up).unwrap();
                                    if let EntityType::Tile(ttype) = ttype.entity_type{
                                        borders.insert(offset, ttype);
                                    }
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