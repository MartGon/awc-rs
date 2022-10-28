use std::fs;

use awc::component::EntityType;
use awc::tile;
use awc::map;
use awc::game::*;
use awc::player::*;

mod spritesheet;
mod tileset;
mod assets;
mod mapview;

use macroquad::window;
use spritesheet::*;

use macroquad::prelude::*;
use tileset::Borders;


#[macroquad::main("BasicShapes")]
async fn main() {

    // Create game
    let mut game = Game::new();
    let _pid = game.create_player(TeamID::new(0));

    // Load map data
    let map_data_str = fs::read_to_string("data/maps/map_data.ron").expect("Could not read map data");
    let map_data = ron::from_str::<map::Data>(&map_data_str).expect("Error on str -> ron");
    game.load_map_data(map_data).expect("Error on loading map data");

    // Load SpriteSheet
    let spritesheet = Image::from_file_with_format(include_bytes!("../../sprites/spritesheet2.png"), Some(ImageFormat::Png));
    let spritesheet = Texture2D::from_image(&spritesheet);
    
    // Load tileset 
    let tileset = tileset::load_from_master_file("sprites/tileset.ron");
    let res = tileset.unwrap();
    let tileset = res.0;

    // Map view
    let mut map_view = mapview::MapView::new( tileset, spritesheet);
    map_view.set_scale(vec2(2.0, 2.0));

    // TODO: Log errors
    for (_id, e) in res.1{
        println!("Error while loading {}", e);
    }

    let tile_size = UVec2::new(16, 16);
    let mut tile_type = tile::TypeID::new(0);
    loop {

        let scale = vec2(2.0, 2.0);
        let draw_size = tile_size.as_vec2() * scale;
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
        
        // Draw Map
        let screen_size = vec2(screen_width(), screen_height());
        let pos = screen_size / 2.0 - map_view.get_size(&game.map, tile_size).as_vec2() / 2.0;
        map_view.draw_map(&game.map, game.components(), tile_size, pos.as_uvec2());
        
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