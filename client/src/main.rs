use awc::tile;
use macroquad::prelude::*;
use macroquad::ui::*;
use awc::{component, component::Component, map};
use awc::game::*;
use awc::player::*;

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
    let grass_rect = Some(Rect::new(238.0, 18.0, 16.0, 16.0));
    let mountain_rect = Some(Rect::new(255.0, 18.0, 16.0, 16.0));
    let water_rect = Some(Rect::new(12.0, 133.0, 16.0, 16.0));

    let tile_size = Vec2::new(32.0, 32.0);
    loop {
        clear_background(RED);
        let x_tiles = (screen_width() / tile_size.x) as i32 + 1;
        let y_tiles = (screen_height() / tile_size.y) as i32 + 1;

        for x in 0..x_tiles{
            for y in 0..y_tiles{
                let draw_pos = Vec2::new(x as f32 * tile_size.x, y as f32 * tile_size.y);
                let dtp = DrawTextureParams{dest_size : Some(tile_size), source : water_rect, rotation : 0.0,  flip_x : false, flip_y : false, pivot : None};
                draw_texture_ex(spritesheet, draw_pos.x, draw_pos.y, WHITE, dtp);
            }
            
        }

        for tile in game.map.tiles(){
            
            let tile_pos = game.components().get_position(tile).unwrap();
            let draw_pos = Vec2::new(tile_pos.pos.x as f32 * tile_size.x, tile_pos.pos.y as f32 * tile_size.y);
            let dtp = DrawTextureParams{dest_size : Some(tile_size), source : grass_rect, rotation : 0.0,  flip_x : false, flip_y : false, pivot : None};
            draw_texture_ex(spritesheet, draw_pos.x, draw_pos.y, WHITE, dtp);
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