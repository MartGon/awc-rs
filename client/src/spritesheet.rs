use std::collections::HashMap;

use macroquad::prelude::*;
use miniquad::FilterMode;
use serde::{Deserialize, Serialize};

const DEFAULT_SPRITE_SIZE : Vec2 = Vec2{x : 16.0, y : 16.0};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Sprite{
    pos : UVec2,
    size : UVec2,
}

impl Sprite{
    pub fn new(pos : UVec2, size : UVec2) -> Self{
        Self{pos, size}
    }

    pub fn new_raw(x : u32, y : u32, w : u32, h : u32) ->Self{
        Self::new(UVec2::new(x, y),  UVec2::new(w, h))
    }
}

impl Sprite{
    pub fn draw(&self, spritesheet : &Texture2D, draw_dest : Vec2) {
        self.draw_scaled(spritesheet, draw_dest, Vec2::new(1.0, 1.0));
    }

    pub fn draw_scaled(&self, spritesheet : &Texture2D, draw_dest : Vec2, scale : Vec2) {
        let rect = Some(Rect::new(self.pos.x as f32, self.pos.y as f32, self.size.x as f32, self.size.y as f32));
        let dtp = DrawTextureParams{dest_size : Some(scale * self.size.as_vec2()), source : rect, rotation : 0.0,  flip_x : false, flip_y : false, pivot : None};
        spritesheet.set_filter(FilterMode::Nearest);
        draw_texture_ex(*spritesheet, draw_dest.x, draw_dest.y, WHITE, dtp)
    }

    pub fn size(&self) -> UVec2{
        self.size
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct AnimatedSprite{
    pub animations : HashMap<String, Animation>
}

impl AnimatedSprite{
    pub fn new(size : UVec2, animations : &[(String, Animation)],) -> AnimatedSprite{
        AnimatedSprite {animations : animations.into_iter().cloned().collect(),}
    }

    pub fn set_cur_anim(&mut self, animation : String)
    {
        self.animations.get(&animation).expect(format!("Could not find animation {animation}").as_str());
    }

    pub fn frame<S: Into<String>>(&self, anim : S) -> Option<Sprite>{
        let str :  String = anim.into();
        if let Some(anim) = self.animations.get(&str)
        {
            let frame_time = get_time();
            let frame_index = (frame_time * anim.fps as f64) as usize % anim.frames.len();
            let frame = &anim.frames[frame_index];

            return Some(Sprite::new(frame.source, anim.size));
        }

        None
    }
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
pub struct Animation{
    pub name : String,
    pub size : UVec2,
    pub frames : Vec<AnimationFrame>,
    pub fps : u32,
}

impl Animation{

    #[allow(dead_code)]
    pub fn new(name : String, size : UVec2, fps : u32, frames : &[AnimationFrame]) -> Animation{
        Animation { name, size, frames: frames.to_vec(), fps: fps }
    }
    
    #[allow(dead_code)]
    pub fn new_short(name : String, size : UVec2, fps : u32, frames_pos : &[UVec2]) -> Animation{
        Animation { name, size, fps : fps, frames : frames_pos.into_iter().map(|x| AnimationFrame::new(x.clone())).collect()}
    }

    pub fn new_shorter_x(name : String, size : UVec2, fps : u32, frame_y : u32, frames_x : &[u32]) -> Animation{
        Animation { name, size, fps : fps, frames : frames_x.into_iter().map(|x| AnimationFrame::new(uvec2(*x, frame_y))).collect()}
    }

    pub fn new_shorter_y(name : String, size : UVec2, fps : u32, frame_x : u32, frames_y : &[u32]) -> Animation{
        Animation { name, size, fps : fps, frames : frames_y.into_iter().map(|y| AnimationFrame::new(uvec2(frame_x, *y))).collect()}
    }
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
pub struct AnimationFrame{
    pub source : UVec2,
}

impl AnimationFrame {
    pub fn new(source : UVec2) -> AnimationFrame{
        AnimationFrame { source }
    }
}