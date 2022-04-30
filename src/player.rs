use raylib::{math::{Vector3, Vector2}, prelude::{RaylibDrawHandle, RaylibDraw}, color::Color, RaylibHandle, consts::KeyboardKey};

use crate::move_point_distance;


pub struct player{
    pos: Vector2,
    rot: f32,
}

impl player{

    pub fn new() -> player{

        player {
            pos: Vector2 { x: 320.0, y: 240.0 },
            rot: 0.0 
            }

    }

    pub fn draw(&self,handle: &mut RaylibDrawHandle) {

        handle.draw_circle(self.pos.x as i32, self.pos.y as i32, 7.0, Color::BLACK);
        handle.draw_circle(self.pos.x as i32, self.pos.y as i32, 6.5, Color::YELLOW);
        
        
    }

    pub fn input(&mut self,handle: &RaylibHandle){
        if handle.is_key_down(KeyboardKey::KEY_W){
            self.pos = move_point_distance(&self.pos,&handle.get_mouse_position(), 3.0);
        }
        if handle.is_key_down(KeyboardKey::KEY_S){
            self.pos = move_point_distance(&self.pos,&handle.get_mouse_position(), -3.0);
        }
    }


}