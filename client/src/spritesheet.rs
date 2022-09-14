
use macroquad::prelude::*;

pub trait Drawable{
    fn draw(&self, spritesheet : &Texture2D, draw_dest : Vec2);
    fn draw_scaled(&self, spritesheet : &Texture2D, draw_dest : Vec2, scale : Vec2);
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
    fn draw(&self, spritesheet : &Texture2D, draw_dest : Vec2) {
        self.draw_scaled(spritesheet, draw_dest, Vec2::new(1.0, 1.0));
    }

    fn draw_scaled(&self, spritesheet : &Texture2D, draw_dest : Vec2, scale : Vec2) {
        let rect = Some(Rect::new(self.pos.x as f32, self.pos.y as f32, self.size.x as f32, self.size.y as f32));
        let dtp = DrawTextureParams{dest_size : Some(scale * self.size.as_f32()), source : rect, rotation : 0.0,  flip_x : false, flip_y : false, pivot : None};
        draw_texture_ex(*spritesheet, draw_dest.x, draw_dest.y, WHITE, dtp)
    }
}

struct AnimatedSprite{
    size : IVec2,
    pos_vec : Vec<IVec2>
}

impl Sprite{

}