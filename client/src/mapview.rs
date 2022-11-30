use std::collections::HashMap;

use awc::{*, component::{EntityType}};
use crate::{glam::{UVec2, ivec2, uvec2}, spritesheet::{Sprite, self}};
use crate::glam;
use macroquad::{texture::Texture2D, shapes::draw_rectangle, prelude::{Color, PINK}};

use crate::{tileset::{self, Borders}, spritesheet::{AnimatedSprite}, unitset};

pub struct MapView{
    spritesheet : Texture2D,
    tileset : tileset::Tileset,
    tile_size : glam::UVec2,

    unitsheet : Texture2D,
    unitset : unitset::UnitSet,

    cam_pos : UVec2,

    highlighted_tiles : HashMap<map::Pos, Color>,
}

impl MapView{

    pub fn new(spritesheet : Texture2D, tileset : tileset::Tileset, tile_size : UVec2, unitsheet : Texture2D, unitset : unitset::UnitSet ) -> MapView{
        MapView { 
            tileset, 
            spritesheet, 
            tile_size, 
            cam_pos : uvec2(0, 0), 
            unitsheet, 
            unitset,
            highlighted_tiles : HashMap::new()
        }
    }

    pub fn _get_pixel_size(&self, map : &map::Map)-> UVec2{
        map.size * self.tile_size
    }

    pub fn set_cam_pos(&mut self, cam_pos : UVec2){
        self.cam_pos = cam_pos
    }

    pub fn get_cam_pos(&self) -> UVec2{
        self.cam_pos
    }

    fn calc_cam_pos(&self, map_size : UVec2, target_size : UVec2) -> UVec2{
        let map_size_pixels = map_size* self.tile_size;

        let fits = map_size_pixels.x <= target_size.x && map_size_pixels.y <= target_size.y;
        if fits {
            uvec2(0, 0)
        } 
        else {
            // Calc camera bounds. This is done to avoid showing blank areas on the right.
            let target_tile_size = (target_size.as_vec2() / self.tile_size.as_vec2()).ceil().as_uvec2();
            let target_tile_size = target_tile_size.clamp(uvec2(1, 1), map_size);
            UVec2::min(self.cam_pos, map_size - target_tile_size)
        }
    }

    pub fn get_map_pos(&self, map_size : UVec2, draw_pos : UVec2, draw_target_size : UVec2, mouse_pos : UVec2) -> Option<UVec2>{

        let ul_corner = draw_pos;
        let dr_corner = draw_pos + draw_target_size;
        let mouse_in_bounds = ul_corner.x <= mouse_pos.x && ul_corner.y <= mouse_pos.y && dr_corner.x >= mouse_pos.x && dr_corner.y >= mouse_pos.y;
        
        if  mouse_in_bounds{

            let mouse_pos = mouse_pos - draw_pos;
            let rel_pos = mouse_pos.as_vec2() / draw_target_size.as_vec2();

            let target_tile_size = draw_target_size.as_vec2() / self.tile_size.as_vec2();
            let tile_pos = (rel_pos * target_tile_size).as_uvec2();

            let cam_pos = self.calc_cam_pos(map_size, draw_target_size);
            let tile_pos = tile_pos + cam_pos;
            return Some(tile_pos);
        }

        None
    }

    pub fn draw_map(&mut self, game : &game::Game, view_pos : UVec2, target_size : UVec2){
        
        // Update cam pos
        self.cam_pos = self.calc_cam_pos(game.map.size, target_size);
        self.draw_tiles(game, view_pos, target_size);
        self.draw_units(game, view_pos, target_size);
    }

    fn draw_tiles(&mut self, game : &game::Game, view_pos : UVec2, target_size : UVec2){
        let map = &game.map;
        let components = game.components();

        for tile in map.tiles(){       
            let tile_pos = components.get_position(&tile).unwrap().pos;     
            if let Some(draw_pos) = self.get_draw_pos(tile_pos, target_size){

                let ttype = components.get_type(tile).unwrap();
                if ttype.is_tile() {
                    if let Some(tile_sprite) = self.tileset.get(&ttype.type_id){

                        // Calculate borders
                        let mut borders = Borders::default();
                        for x in tileset::OFFSET_MIN..tileset::OFFSET_MAX + 1{
                            for y in tileset::OFFSET_MIN..tileset::OFFSET_MAX + 1{
                                let offset = ivec2(x, y);

                                let pos = tile_pos.as_ivec2() + offset;
                                if pos.x >= 0 && pos.y >= 0 {
                                    if let Some(up) = map.get_tile_in_pos(&components, &pos.as_uvec2()){
                                        let ttype = components.get_type(&up).unwrap();
                                        if let EntityType::Tile = ttype.entity_type{
                                            borders.insert(offset, ttype.type_id);
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Draw tile
                        let sprite = tile_sprite.sprite(&borders);
                        let draw_pos = view_pos + draw_pos;
                        let scale = self.tile_size.as_vec2() / sprite.size().as_vec2();
                        sprite.draw_scaled(&self.spritesheet, draw_pos.as_vec2(), scale);
                        
                        // Highlight tile
                        if let Some(color) = self.highlighted_tiles.get(&tile_pos){
                            draw_rectangle(draw_pos.x as f32, draw_pos.y as f32, self.tile_size.x as f32, self.tile_size.y as f32, *color);
                        }
                    }
                }
            }
        }
    }

    fn draw_units(&mut self, game : &game::Game, view_pos : UVec2, target_size : UVec2){
        let map = &game.map;
        let components = game.components();

        for unit in map.units(){
            
            let unit_pos = components.get_position(&unit).unwrap().pos;
            if let Some(draw_pos) = self.get_draw_pos(unit_pos, target_size){
                let utype = components.get_type(unit).unwrap();
                if utype.is_unit(){
                    if let Some(unit_sprite) = self.unitset.get(&utype.type_id){
                        
                        let owner = components.get_owner(unit).expect("A unit didn't have an owner");
                        let owner = game.get_player(&owner.owner).expect("Could not find player in game by component's id");
                        
                        let mut sprite : Option<Sprite> = None;
                        if let Some(animated_sprite) = unit_sprite.sprite_faction(owner.team, owner.faction){
                            
                            sprite = animated_sprite.frame("idle");

                            if let Some(turn) = game.get_turn(){
                                if turn.is_waiting(*unit){
                                    sprite = animated_sprite.frame("wait");
                                }
                            }
                        }
                        
                        let draw_pos = view_pos + draw_pos;
                        if let Some(sprite) = sprite{
                            let scale = self.tile_size.as_vec2() / sprite.size().as_vec2();
                            sprite.draw_scaled(&self.unitsheet, draw_pos.as_vec2(), scale);
                        }
                        // Draw pink rectangle
                        else{
                            draw_rectangle(draw_pos.x as f32, draw_pos.y as f32, self.tile_size.x as f32, self.tile_size.y as f32, PINK);
                        }
                    }
                }
            }
        }
    }

    fn get_draw_pos(&self, tile_pos : UVec2, target_size : UVec2) -> Option<UVec2>{
            
        // Skip tiles on the left/up that'll never be drawn
        // Would cause an overflow error otherwise.
        if tile_pos.x < self.cam_pos.x || tile_pos.y < self.cam_pos.y {return None;}
        
        let draw_pos = (tile_pos - self.cam_pos) * self.tile_size;
        
        // Skip tiles outside of target size
        if draw_pos.x >= target_size.x || draw_pos.y >= target_size.y {return None;}

        Some(draw_pos)
    }

    pub fn highlight_tiles(&mut self, tiles : Vec<map::Pos>, color : Color){
        for tile in tiles{
            self.highlighted_tiles.insert(tile, color);
        }
    }

    pub fn clear_highlighted_tiles(&mut self){
        self.highlighted_tiles.clear();
    }
    
}