
use std::ops::Index;

use macroquad::prelude::{*, cast::IVec2Cast};

pub trait Drawable{
    fn draw(&mut self, spritesheet : &Texture2D, draw_dest : Vec2);
    fn draw_scaled(&mut self, spritesheet : &Texture2D, draw_dest : Vec2, scale : Vec2);
}

pub struct Sprite{
    pos : IVec2,
    size : IVec2,
}

impl Sprite{
    pub fn new(pos : IVec2, size : IVec2) -> Self{
        Self{pos, size}
    }

    pub fn new_raw(x : i32, y : i32, w : i32, h : i32) ->Self{
        Self::new(IVec2::new(x, y),  IVec2::new(w, h))
    }
}

impl Drawable for Sprite{
    fn draw(&mut self, spritesheet : &Texture2D, draw_dest : Vec2) {
        self.draw_scaled(spritesheet, draw_dest, Vec2::new(1.0, 1.0));
    }

    fn draw_scaled(&mut self, spritesheet : &Texture2D, draw_dest : Vec2, scale : Vec2) {
        let rect = Some(Rect::new(self.pos.x as f32, self.pos.y as f32, self.size.x as f32, self.size.y as f32));
        let dtp = DrawTextureParams{dest_size : Some(scale * self.size.as_f32()), source : rect, rotation : 0.0,  flip_x : false, flip_y : false, pivot : None};
        draw_texture_ex(*spritesheet, draw_dest.x, draw_dest.y, WHITE, dtp)
    }
}

pub struct AnimatedSprite{
    pub size : UVec2,
    pub animations : Vec<Animation>,

    current_animation : usize,
    frame : usize,
}

impl AnimatedSprite{
    pub fn new(size : UVec2, animations : &[Animation],) -> AnimatedSprite{
        AnimatedSprite {size, animations : animations.to_vec(), current_animation : 0, frame : 0}
    }

    pub fn frame(&self) -> &AnimationFrame{
        let anim = &self.animations[self.current_animation];
        let frame_time = get_time();
        let frame = (frame_time * anim.fps as f64) as usize % anim.frames.len();

        &anim.frames[frame]
    }

    pub fn update(&mut self){

    }
}

impl Drawable for AnimatedSprite{
    fn draw(&mut self, spritesheet : &Texture2D, draw_dest : Vec2) {
        self.draw_scaled(spritesheet, draw_dest, Vec2::new(1.0, 1.0));
    }

    fn draw_scaled(&mut self, spritesheet : &Texture2D, draw_dest : Vec2, scale : Vec2) {
        let frame = self.frame();        
        let rect = Some(Rect::new(frame.source.x as f32, frame.source.y as f32, self.size.x as f32, self.size.y as f32));
        let dtp = DrawTextureParams{dest_size : Some(scale * self.size.as_f32()), source : rect, rotation : 0.0,  flip_x : false, flip_y : false, pivot : None};

        self.update();
        draw_texture_ex(*spritesheet, draw_dest.x, draw_dest.y, WHITE, dtp)
    }
}

#[derive(Clone)]
pub struct Animation{
    pub name : String,
    pub frames : Vec<AnimationFrame>,
    pub fps : u32,
}

impl Animation{
    pub fn new(name : String, fps : u32, frames : &[AnimationFrame]) -> Animation{
        Animation { name: name, frames: frames.to_vec(), fps: fps }
    }
}

#[derive(Clone)]
pub struct AnimationFrame{
    pub source : IVec2,
}

impl AnimationFrame {
    pub fn new(source : IVec2) -> AnimationFrame{
        AnimationFrame { source }
    }
}