use macroquad::prelude::*;
use miniquad::FilterMode;
use serde::{Deserialize, Serialize};

pub trait Drawable{
    fn draw(&self, spritesheet : &Texture2D, draw_dest : Vec2);
    fn draw_scaled(&self, spritesheet : &Texture2D, draw_dest : Vec2, scale : Vec2);

    fn draw_mut(&mut self, spritesheet : &Texture2D, draw_dest : Vec2);
    fn draw_mut_scaled(&mut self, spritesheet : &Texture2D, draw_dest : Vec2, scale : Vec2);
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum Sprite{
    Idle(IdleSprite),
    Animated(AnimatedSprite)
}

pub fn sprite(pos : UVec2, size : UVec2) -> Sprite{
    Sprite::Idle(IdleSprite::new(pos, size))
}

pub fn sprite_raw(x : u32, y : u32, w : u32, h : u32) -> Sprite{
    Sprite::Idle(IdleSprite::new_raw(x, y, w, h))
}

pub fn animated_sprite(size : UVec2, animations : &[Animation],) -> Sprite{
    Sprite::Animated(AnimatedSprite::new(size, animations))
}

impl Default for Sprite{
    fn default() -> Self {
        Self::Idle(IdleSprite::new(uvec2(0, 0), uvec2(1, 1)))
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct IdleSprite{
    pos : UVec2,
    size : UVec2,
}

impl IdleSprite{
    pub fn new(pos : UVec2, size : UVec2) -> Self{
        Self{pos, size}
    }

    pub fn new_raw(x : u32, y : u32, w : u32, h : u32) ->Self{
        Self::new(UVec2::new(x, y),  UVec2::new(w, h))
    }
}

impl Drawable for IdleSprite{
    fn draw(&self, spritesheet : &Texture2D, draw_dest : Vec2) {
        self.draw_scaled(spritesheet, draw_dest, Vec2::new(1.0, 1.0));
    }

    fn draw_scaled(&self, spritesheet : &Texture2D, draw_dest : Vec2, scale : Vec2) {
        let rect = Some(Rect::new(self.pos.x as f32, self.pos.y as f32, self.size.x as f32, self.size.y as f32));
        let dtp = DrawTextureParams{dest_size : Some(scale * self.size.as_vec2()), source : rect, rotation : 0.0,  flip_x : false, flip_y : false, pivot : None};
        spritesheet.set_filter(FilterMode::Nearest);
        draw_texture_ex(*spritesheet, draw_dest.x, draw_dest.y, WHITE, dtp)
    }

    fn draw_mut(&mut self, spritesheet : &Texture2D, draw_dest : Vec2) {
        todo!()
    }

    fn draw_mut_scaled(&mut self, spritesheet : &Texture2D, draw_dest : Vec2, scale : Vec2) {
        todo!()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AnimatedSprite{
    pub size : UVec2,
    pub animations : Vec<Animation>,
}

impl AnimatedSprite{
    pub fn new(size : UVec2, animations : &[Animation],) -> AnimatedSprite{
        AnimatedSprite {size, animations : animations.to_vec()}
    }

    pub fn frame(&self) -> &AnimationFrame{
        let anim = &self.animations[0];
        let frame_time = get_time();
        let frame = (frame_time * anim.fps as f64) as usize % anim.frames.len();

        &anim.frames[frame]
    }

    pub fn update(&mut self){

    }
}

impl Drawable for AnimatedSprite{
    fn draw_mut(&mut self, spritesheet : &Texture2D, draw_dest : Vec2) {
        self.draw_mut_scaled(spritesheet, draw_dest, Vec2::new(1.0, 1.0));
    }

    fn draw(&self, spritesheet : &Texture2D, draw_dest : Vec2) {
        self.draw_scaled(spritesheet, draw_dest, Vec2::new(1.0, 1.0));
    }

    fn draw_scaled(&self, spritesheet : &Texture2D, draw_dest : Vec2, scale : Vec2) {
        let frame = self.frame();        
        let rect = Some(Rect::new(frame.source.x as f32, frame.source.y as f32, self.size.x as f32, self.size.y as f32));
        let dtp = DrawTextureParams{dest_size : Some(scale * self.size.as_vec2()), source : rect, rotation : 0.0,  flip_x : false, flip_y : false, pivot : None};

        spritesheet.set_filter(FilterMode::Nearest);
        draw_texture_ex(*spritesheet, draw_dest.x, draw_dest.y, WHITE, dtp)
    }

    fn draw_mut_scaled(&mut self, spritesheet : &Texture2D, draw_dest : Vec2, scale : Vec2) {
        todo!()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Animation{
    pub name : String,
    pub frames : Vec<AnimationFrame>,
    pub fps : u32,
}

impl Animation{

    #[allow(dead_code)]
    pub fn new(name : String, fps : u32, frames : &[AnimationFrame]) -> Animation{
        Animation { name, frames: frames.to_vec(), fps: fps }
    }
    
    #[allow(dead_code)]
    pub fn new_short(name : String, fps : u32, frames_pos : &[UVec2]) -> Animation{
        Animation { name, fps : fps, frames : frames_pos.into_iter().map(|x| AnimationFrame::new(x.clone())).collect()}
    }

    pub fn new_shorter_x(name : String, fps : u32, frame_y : u32, frames_x : &[u32]) -> Animation{
        Animation { name, fps : fps, frames : frames_x.into_iter().map(|x| AnimationFrame::new(uvec2(*x, frame_y))).collect()}
    }

    pub fn new_shorter_y(name : String, fps : u32, frame_x : u32, frames_y : &[u32]) -> Animation{
        Animation { name, fps : fps, frames : frames_y.into_iter().map(|y| AnimationFrame::new(uvec2(frame_x, *y))).collect()}
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AnimationFrame{
    pub source : UVec2,
}

impl AnimationFrame {
    pub fn new(source : UVec2) -> AnimationFrame{
        AnimationFrame { source }
    }
}