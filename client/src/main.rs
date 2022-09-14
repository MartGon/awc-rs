use awc::tile;
use macroquad::prelude::*;
use macroquad::ui::*;
use awc::{component, component::Component, map};
use awc::game::*;
use awc::player::*;

mod spritesheet;
use spritesheet::*;

#[macroquad::main("BasicShapes")]
async fn main() {

    let mut game = Game::new();
    let pid = game.create_player(TeamID::new(0));

    // Init map
    let land_size = IVec2::new(4, 4);
    for x in 0..land_size.x{
        for y in 0..land_size.y{
            let tile_id = game.create_tile(tile::TypeID::new(0));
            let pos = component::Position{pos : map::Pos{x : x, y : y, z : 0}};
            game.insert_component(tile_id, Component::Position(pos));
        }
    }

    // Load SpriteSheet
    let spritesheet = Image::from_file_with_format(include_bytes!("../../sprites/spritesheet.png"), Some(ImageFormat::Png));
    let spritesheet = Texture2D::from_image(&spritesheet);

    // Tile Sprites
    let grass = Sprite::new_raw(238, 18, 16, 16);
    let mountain = Sprite::new_raw(255, 18, 16, 16);
    let water = Sprite::new_raw(12, 133, 16, 16);

    let tile_size = Vec2::new(32.0, 32.0);
    loop {
        clear_background(RED);
        let x_tiles = (screen_width() / tile_size.x) as i32 + 1;
        let y_tiles = (screen_height() / tile_size.y) as i32 + 1;

        for x in 0..x_tiles{
            for y in 0..y_tiles{
                let draw_pos = Vec2::new(x as f32 * tile_size.x, y as f32 * tile_size.y);
                water.draw_scaled(&spritesheet, draw_pos, Vec2::new(2.0, 2.0));
            }
            
        }

        for tile in game.map.tiles(){
            
            let tile_pos = game.components().get_position(tile).unwrap();
            let draw_pos = Vec2::new(tile_pos.pos.x as f32 * tile_size.x, tile_pos.pos.y as f32 * tile_size.y);
            grass.draw_scaled(&spritesheet, draw_pos, Vec2::new(2.0, 2.0));
        }
        
        // Draw UI
        if root_ui().window(1, Vec2::new(screen_width() / 2.0, screen_height() / 2.0), Vec2::new(86.0, 64.0), |ui|{
            if ui.button(None, "Push me") {
                println!("pushed");
            }}){

        }

        next_frame().await
    }
}