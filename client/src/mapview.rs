use awc::{*, component::EntityType};
use glam::{Vec2, vec2, UVec2, ivec2, uvec2};
use macroquad::texture::Texture2D;

use crate::{tileset::{self, Borders}, spritesheet::{self, Drawable}};

pub struct MapView{
    spritesheet : Texture2D,
    tileset : tileset::Tileset,
    tile_size : UVec2,
    cam_pos : UVec2
}

impl MapView{

    pub fn new(spritesheet : Texture2D, tileset : tileset::Tileset, tile_size : UVec2) -> MapView{
        MapView { tileset, spritesheet, tile_size, cam_pos : uvec2(0, 0)}
    }

    pub fn get_size(&self, map : &map::Map)-> UVec2{
        map.size * self.tile_size
    }

    pub fn set_cam_pos(&mut self, cam_pos : UVec2){
        self.cam_pos = cam_pos
    }

    pub fn get_cam_pos(&self) -> UVec2{
        self.cam_pos
    }

    pub fn draw_map(&mut self, map : &map::Map, components : &component::Components, pos : UVec2, target_size : UVec2){
        let tile_draw_size = self.tile_size.as_vec2();
        let map_size = self.get_size(map);

        let fits = map_size.x <= target_size.x && map_size.y <= target_size.y;
        self.cam_pos = if fits {
            uvec2(0, 0)
        } 
        else {
            // Calc camera bounds. This is done to avoid showing blank areas on the right.
            let target_tile_size = (target_size.as_vec2() / self.tile_size.as_vec2()).ceil().as_uvec2();
            let target_tile_size = target_tile_size.clamp(uvec2(1, 1), map.size);
            self.cam_pos.min(map.size - target_tile_size)
        };

        for tile in map.tiles(){            
            let tile_pos = components.get_position(tile).unwrap().pos;
            
            // Skip tiles on the left/up that'll never be drawn
            // Would cause an overflow error otherwise.
            if tile_pos.x < self.cam_pos.x || tile_pos.y < self.cam_pos.y {continue;}
            
            let draw_pos = tile_pos - self.cam_pos;
            let draw_pos = pos.as_vec2() + draw_pos.as_vec2() * tile_draw_size;

            let ttype = components.get_type(tile).unwrap();
            if let EntityType::Tile(ttype) = ttype.entity_type {
                if let Some(tile_sprite) = self.tileset.get(&ttype){

                    // Calculate borders
                    let mut borders = Borders::default();
                    
                    for x in tileset::OFFSET_MIN..tileset::OFFSET_MAX + 1{
                        for y in tileset::OFFSET_MIN..tileset::OFFSET_MAX + 1{
                            let offset = ivec2(x, y);

                            let pos = tile_pos.as_ivec2() + offset;
                            if pos.x >= 0 && pos.y >= 0 {
                                if let Some(up) = map.get_tile_in_pos(&components, &pos.as_uvec2()){
                                    let ttype = components.get_type(&up).unwrap();
                                    if let EntityType::Tile(ttype) = ttype.entity_type{
                                        borders.insert(offset, ttype);
                                    }
                                }
                            }
                        }
                    }
                    
                    // Draw tile
                    let sprite = tile_sprite.sprite(&borders);
                    let scale = tile_draw_size / sprite.size().as_vec2();
                    sprite.draw_scaled(&self.spritesheet, draw_pos, scale);
                }
            }
        }
    }
    
}