use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

use assets::MasterFile;
use awc::command::Command;
use awc::command;
use awc::component::EntityType;
use awc::tile;
use awc::map;
use awc::game::*;
use awc::player::*;
use awc::unit;
use awc::weapon;
use awc::movement;

mod spritesheet;
mod tileset;
mod unitset;
mod assets;
mod mapview;

use macroquad::prelude::*;
use mlua::Lua;

type UnitTemplates = HashMap<unit::TypeID, unit::Template>;
type Scripts = HashMap<String, String>;

impl MasterFile<unit::Template> for UnitTemplates{}
impl MasterFile<String> for Scripts{}

#[macroquad::main("BasicShapes")]
async fn main() {

    // Debug
    env_logger::init();
    env::set_var("RUST_LOG", "error");
    env::set_var("RUST_BACKTRACE", "1");


    let mut lua = Lua::new();
    {
        // Create game
        let mut game = Game::new(&mut lua);
        let p1 = game.create_player(Team::Red, Faction::OrangeStar);
        let p2 = game.create_player(Team::Blue, Faction::BlueMoon);
        let p3 = game.create_player(Team::Red, Faction::BlueMoon);
        println!("Player 1 id: {:?}", p1);
        println!("Player 2 id: {:?}", p2);

        // Load game data
        let infantry_weapon = weapon::Weapon::new(weapon::Range::new(0, 1), 20, 99, &[(0.into(), true), (1.into(), true)]);
        let infantry_movement = movement::Movement::new(3, &[(1.into(), 1), (2.into(), 2)]);

            // Load Scripts
        let scripts = assets::load_master_file::<String, &str>("data/scripts.ron").unwrap();
        for (id, script_path) in scripts{
            game.load_script(&id, script_path).expect("Something failed");
            println!("Loaded script: {}", id);
        }
        
            // Load unit templates
        let unit_templates = awc::unit::Template::load_from_master_file::<unit::TypeID, &str>("data/units.ron", unit::Template::new(&[], None));
        let res = unit_templates.unwrap();
        for (_id, e) in res.1{
            log::error!("Error while loading {}", e);
        }
        for (id, unit_template) in res.0{
            game.add_unit_template(id, unit_template)
        }

        // Load map data
        let map_data_str = fs::read_to_string("data/maps/map_data2.ron").expect("Could not read map data");
        let map_data = ron::from_str::<map::Data>(&map_data_str).expect("Error on str -> ron");
        game.load_map_data(map_data).expect("Error on loading map data");

        // Load SpriteSheet
        let tilesheet = Image::from_file_with_format(include_bytes!("../../sprites/spritesheet2.png"), Some(ImageFormat::Png));
        let tilesheet = Texture2D::from_image(&tilesheet);
        
        // Load tileset 
        let tileset = tileset::BorderedTile::load_from_master_file_default("sprites/tileset.ron");
        let res = tileset.unwrap();
        for (_id, e) in res.1{
            log::error!("Error while loading {}", e);
        }
        let tileset = res.0;

        // Write map
        /* 
        let map_data = game.get_map_data([(ID::new(0), ' '), (ID::new(1), '#'), (ID::new(2), 'M')].into_iter().collect());
        let map_data_ron = ron::ser::to_string_pretty(&map_data, ron::ser::PrettyConfig::default()).expect("Error on ser");
        fs::write("data/maps/map_data2.ron", map_data_ron).expect("Error on write");
        */

        // Load unit sheet
        let mut unitsheet = Image::from_file_with_format(include_bytes!("../../sprites/unitsheet.png"), Some(ImageFormat::Png));
        for y in 0..unitsheet.height() as u32 { for x in 0..unitsheet.width() as u32 { if unitsheet.get_pixel(x, y) == Color::from_rgba(255, 127, 255, 255){ unitsheet.set_pixel(x, y, Color::from_rgba(0, 0, 0, 0))}}};
        let unitsheet = Texture2D::from_image(&unitsheet);

        // Load UnitSet
        let unitset = unitset::Unit::load_from_master_file_default("sprites/unitset.ron");
        let res = unitset.unwrap();
        for (_id, e) in res.1{
            log::error!("Error while loading {}", e);
        }
        let unitset = res.0;

        // Map view
        let tile_size = UVec2::new(64, 64);
        let mut map_view = mapview::MapView::new(tilesheet, tileset, tile_size, unitsheet, unitset);

        let mut tile_type = tile::TypeID::new(0);
        let mut move_unit : Option<awc::ID> = None;

        // Start game
        game.start();

        loop {

            let screen_size = vec2(screen_width(), screen_height());
            let _target_size = uvec2(256, 256);
            let target_size = screen_size.as_uvec2();
            let pos = (screen_size / 2.0 - target_size.as_vec2() / 2.).as_uvec2();

            // Inpput handling \\
            let (x, y) = mouse_position();
            let mouse_pos = uvec2(x as u32, y as u32);
            let cur_player = game.current_turn().player;
            if let Some(map_pos) = map_view.get_map_pos(game.map.size, pos, target_size, mouse_pos)
            {     
                if is_mouse_button_released(MouseButton::Left){
                    
                    // Select Unit
                    if let Some(unit) = game.get_unit_in_pos(&map_pos){
                        move_unit = Some(unit);
                        println!("Selected unit {:?}", unit);

                        if let Ok(area) = game.calc_move_area(unit){
                            map_view.highlight_tiles(vec!(area.get_origin()), Color::from_rgba(10, 10, 180, 50));
                            map_view.highlight_tiles(area.get_tiles(), Color::from_rgba(10, 10, 180, 50));
                        }
                    }
                    // Apply movement
                    else if let Some(tile) = game.get_tile_in_pos(&map_pos){

                        if let Some(u) = move_unit{
                            let path = game.calc_path(u, map_pos);
                            println!("Path to {} is: {:?}", map_pos, path);

                            if path.is_ok() {
                                println!("Moving unit");
                                let command = Command::Move(command::Move::new(u, map_pos));
                                if let Err(res) = game.run_command(command, &cur_player){
                                    println!("Error on running last cmd {:?}", res);
                                }
                                else{
                                    let wait = Command::Wait(command::Wait::new(u));
                                    if let Err(res) = game.run_command(wait, &cur_player){
                                        println!("Error on waiting unit {:?}", res);
                                    }
                                    else{
                                        println!("Unit {:?} is now waiting", u);
                                    }
                                }
                            }

                            map_view.clear_highlighted_tiles();
                            move_unit = None;
                        }

                        /* 
                        let entity_type = game.components_mut().get_type_mut(&tile).unwrap();
                        entity_type.type_id = tile_type;
                        */
                    }    
                }

                if is_mouse_button_released(MouseButton::Right){
                    if let Some(tile) = game.get_tile_in_pos(&map_pos){
                        let entity_type = game.components_mut().get_type_mut(&tile).unwrap();
                        if let EntityType::Tile = entity_type.entity_type{
                            tile_type = entity_type.type_id;
                        }
                    }
                }

                if is_key_released(KeyCode::S){
                    if let Some(tile) = game.get_tile_in_pos(&map_pos){
                        if let Err(err) = game.create_unit(Some(0.into()), map_pos, 0.into()){
                            println!("Could not create unit {:?}", err);
                        }
                    }
                }

                if is_key_released(KeyCode::Enter){

                    let turn = game.current_turn();
                    println!("Current player {:?}, Current day {}", turn.player, turn.day);

                    let command = Command::EndTurn(command::EndTurn::new());
                    if let Err(res) = game.run_command(command, &cur_player){
                        println!("Error on running last cmd {:?}", res);
                    }
                    else{
                        let turn = game.current_turn();
                        println!("Next player {:?}, Next day {}", turn.player, turn.day);
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
            map_view.draw_map(&game, pos, target_size);

            draw_rectangle(0., 0., 64., 64., Color::from_rgba(0, 0, 0, 255));
            
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

}