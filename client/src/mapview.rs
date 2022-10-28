use awc::{*, component::EntityType};
use glam::{Vec2, vec2, UVec2, ivec2};
use macroquad::texture::Texture2D;

use crate::{tileset::{self, Borders}, spritesheet::{self, Drawable}};

pub struct MapView{
    tileset : tileset::Tileset,
    spritesheet : Texture2D,
    scale : Vec2,
}

impl MapView{

    pub fn new(tileset : tileset::Tileset, spritesheet : Texture2D) -> MapView{
        MapView { tileset, spritesheet, scale : vec2(1.0, 1.0) }
    }

    // TODO: Tile size could be part of Tileset struct
    pub fn get_size(&self, map : &map::Map, tile_size : UVec2)-> UVec2{
        ((map.size * tile_size).as_vec2() * self.scale).as_uvec2()
    }

    pub fn set_scale(&mut self, scale : Vec2){
        self.scale = scale;
    }

    pub fn draw_map(&self, map : &map::Map, components : &component::Components, tile_size : UVec2, pos : UVec2){
        for tile in map.tiles(){            
            let tile_pos = components.get_position(tile).unwrap().pos.as_ivec2();
            //let tile_pos = awc::map::Pos::new(1, 2, 0);
            let draw_size = tile_size.as_vec2() * self.scale;
            let draw_pos = pos.as_vec2() + Vec2::new(tile_pos.x as f32 * draw_size.x, tile_pos.y as f32 * draw_size.y);

            let ttype = components.get_type(tile).unwrap();
            if let EntityType::Tile(ttype) = ttype.entity_type {
                if let Some(tile_sprite) = self.tileset.get(&ttype){

                    // Calculate borders
                    let mut borders = Borders::default();
                    for x in tileset::OFFSET_MIN..tileset::OFFSET_MAX + 1{
                        for y in tileset::OFFSET_MIN..tileset::OFFSET_MAX + 1{
                            let offset = ivec2(x, y);
                            let pos = tile_pos + offset;
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
                    sprite.draw_scaled(&self.spritesheet, draw_pos, self.scale);
                }
            }
        }
    }
    
}