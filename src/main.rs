use raylib::prelude::*;

mod player;
mod map;


pub fn move_point_distance(p1: &Vector2,p2: &Vector2,dis: f32)-> Vector2{

    let vector = Vector2 { x: p2.x - p1.x, y: p2.y - p1.y };
    let length = (vector.x * vector.x + vector.y * vector.y).sqrt();
    let unit_vector = Vector2::new(vector.x / length, vector.y / length);
    return Vector2::new(p1.x + unit_vector.x * dis, p1.y + unit_vector.y * dis);

}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);
    
    //init stuff
    let mut player = player::player::new();

    while !rl.window_should_close() {

        player.input(&mut rl);

        let mut d = rl.begin_drawing(&thread);
        let mut i = 0;

        while i < 144 {
            if map::map_data[i] == 1{
                let x = (i % 12) as i32;
                let y = (i / 12) as i32;
                d.draw_rectangle(x*40, y*40, 40, 40, Color::BLACK);
            }
            i=i+1;
        }

        d.clear_background(Color::WHITE);
        
        player.draw(&mut d);

    }
}