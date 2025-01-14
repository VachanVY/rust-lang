#![allow(warnings)]

use rand::Rng;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 1000;

struct Ball {
    position: Vector2,
    speed: f32,
    radius: f32,
    color: Color
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Enigma")
        .vsync()
        .build();
    rl.set_target_fps(60);

    let mut ball = Ball {
        position: Vector2::new((SCREEN_WIDTH as f32)/2.0, (SCREEN_HEIGHT as f32)/2.0),
        speed: 3.0,
        radius: 40.0,
        color: Color::GREEN
    };
    
    let mut frame_count = 0;
    let mut rng = rand::thread_rng();
    let mut xval = 0; let mut yval = 0;
    let mut pause = false;

    while !rl.window_should_close() {
        // --- UPDATE --- //
        frame_count += 1;
        if frame_count % 60 == 0 { // one second has passed
            (xval, yval) = rand_range(&mut rng);
            frame_count = 0;
        }
        ball.position = ball.position.lerp(
            Vector2::new(xval as f32, yval as f32),
            0.025
        );

        if rl.is_key_down(KEY_RIGHT) {
            ball.position.x += ball.speed;
        }
        if rl.is_key_down(KEY_LEFT) {
            ball.position.x -= ball.speed;
        }
        if rl.is_key_down(KEY_UP) {
            ball.position.y -= ball.speed;
        }
        if rl.is_key_down(KEY_DOWN) {
            ball.position.y += ball.speed;
        }

        if rl.is_key_pressed(KEY_SPACE) {
            if ball.color == Color::GREEN {
                ball.color = Color::RED;
            }
            else if ball.color == Color::RED {
                ball.color = Color::GREEN;
            }
        }

        if rl.is_mouse_button_down(MOUSE_BUTTON_LEFT) {
            // Detect if a mouse button is being pressed.
            ball.position = ball.position.lerp(
                rl.get_mouse_position(), 
                0.025
            ); // Returns a new Vector2 with componenets linearly interpolated by amount towards vector v
        }
        ball.position.x = ball.position.x.clamp(
            ball.radius,
            SCREEN_WIDTH as f32 - ball.radius
        );
        ball.position.y = ball.position.y.clamp(
            ball.radius,
            SCREEN_HEIGHT as f32 - ball.radius
        );
        // --- UPDATE --- //

        // --- DRAW --- //
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);
        d.draw_text(
            format!("HELLO: {}!", (xval+yval)/2).as_str(),
            xval, yval, 20, Color::WHITE
        );
        d.draw_circle(
            ball.position.x as i32, 
            ball.position.y as i32, 
            ball.radius,
            ball.color
        );
        // --- DRAW --- //
    }
}

fn rand_range(rng: &mut rand::rngs::ThreadRng) -> (i32, i32) {
    let xval = rng.gen_range(0..=SCREEN_WIDTH-100);
    let yval = rng.gen_range(0..=SCREEN_HEIGHT-20);
    return (xval, yval)
}

