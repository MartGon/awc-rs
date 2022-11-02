use std::env;
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

use macroquad::prelude::*;


#[macroquad::main("BasicShapes")]
async fn main() {

    // Debug
    env::set_var("RUST_BACKTRACE", "1");

    // Create game
    let mut game = Game::new();
    let _pid = game.create_player(TeamID::new(0));

    // Load map data
    let map_data_str = fs::read_to_string("data/maps/map_data2.ron").expect("Could not read map data");
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
    let tile_size = UVec2::new(64, 64);
    let mut map_view = mapview::MapView::new(spritesheet, tileset, tile_size);

    // TODO: Log errors
    for (_id, e) in res.1{
        println!("Error while loading {}", e);
    }

    let mut tile_type = tile::TypeID::new(0);
    loop {

        let screen_size = vec2(screen_width(), screen_height());
        let target_size = uvec2(256, 256);
        let target_size = screen_size.as_uvec2();
        let pos = (screen_size / 2.0 - target_size.as_vec2() / 2.).as_uvec2();

        // Inpput handling \\
        // TODO: Get tile pos click from Map View. It's altered by camera, after all
        let (x, y) = mouse_position();
        let mouse_pos = uvec2(x as u32, y as u32);
        if let Some(tile_pos) = map_view.get_tile_pos(game.map.size, pos, target_size, mouse_pos)
        {

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
        }

        // Camera controls
        let mut cam_pos = map_view.get_cam_pos();
        if is_key_released(KeyCode::Left){
            cam_pos.x = if cam_pos.x > 0 {cam_pos.x - 1} else {cam_pos.x};
            map_view.set_cam_pos(cam_pos);
            println!("Left pressed: {}", cam_pos);
        }

        if is_key_released(KeyCode::Right){
            cam_pos.x = cam_pos.x + 1;
            map_view.set_cam_pos(cam_pos);
            println!("Right pressed: {}", cam_pos);
        }

        if is_key_released(KeyCode::Up){
            cam_pos.y = if cam_pos.y > 0 {cam_pos.y - 1} else {cam_pos.y};
            map_view.set_cam_pos(cam_pos);
            println!("Left pressed: {}", cam_pos);
        }

        if is_key_released(KeyCode::Down){
            cam_pos.y = cam_pos.y + 1;
            map_view.set_cam_pos(cam_pos);
            println!("Right pressed: {}", cam_pos);
        }

        // Drawing \\
        clear_background(RED);
        
        // Draw Map
        map_view.draw_map(&game.map, game.components(), pos, target_size);
        
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