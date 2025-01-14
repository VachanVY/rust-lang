#![allow(warnings)]

use rand::Rng;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 800;

struct Ball {
    position: Vector2,
    speed: Vector2,
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

    let mut rng = rand::thread_rng();
    let speed = 4 as f32;
    let mut balls = vec![];
    let colors = [
        Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW, Color::PURPLE,
        Color::DARKCYAN, Color::ORANGE, Color::PINK, Color::GOLD, Color::LIME,
        Color::BEIGE, Color::MAROON, Color::MAGENTA, Color::BROWN, Color::LIGHTGRAY,
    ];

    let image = rl.load_texture(&thread, "Ben10!!!.png").unwrap();

    for i in 0..50 {
        balls.push(Ball {
            position: Vector2::new(rng.gen_range(0..SCREEN_WIDTH) as f32, rng.gen_range(0..SCREEN_HEIGHT) as f32),
            speed: Vector2::new(rng.gen_range(1..=4) as f32, rng.gen_range(1..=4) as f32),
            radius: 40.0,
            color: colors[i % colors.len()],
        });
    }
    
    let mut frame_count = 0;
    let mut pause = false;

    while !rl.window_should_close() {
        // --- UPDATE --- //
        frame_count += 1;
        if frame_count % 60 == 0 { // one second has passed
            frame_count = 0;
        }

        if rl.is_key_pressed(KEY_SPACE) {
            pause = !pause;
            for ball in &mut balls {
                if ball.color == Color::GREEN {
                    ball.color = Color::RED;
                } else if ball.color == Color::RED {
                    ball.color = Color::GREEN;
                }
            }
        }

        if !pause {
            for ball in &mut balls {
                ball.position += ball.speed;
                if ball.position.x >= (SCREEN_WIDTH as f32) - ball.radius // right extreme
                || ball.position.x <= ball.radius { // left extreme
                    ball.speed.x *= -1.0;
                }
                if ball.position.y >= (SCREEN_HEIGHT as f32) - ball.radius
                || ball.position.y <= ball.radius {
                    ball.speed.y *= -1.0;
                }
            }
        }

        if rl.is_mouse_button_down(MOUSE_BUTTON_LEFT) {
            let mouse_position = rl.get_mouse_position();
            for ball in &mut balls {
                ball.position = ball.position.lerp(mouse_position, 0.025);
            }
        }

        for ball in &mut balls {
            ball.position.x = ball.position.x.clamp(ball.radius, SCREEN_WIDTH as f32 - ball.radius);
            ball.position.y = ball.position.y.clamp(ball.radius, SCREEN_HEIGHT as f32 - ball.radius);
        }
        // --- UPDATE --- //

        // --- DRAW --- //
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);
        d.draw_text(
            "Hello",
            SCREEN_WIDTH/2, SCREEN_HEIGHT/2,
            20,
            Color::WHITE
        );
        d.draw_texture_v(
            &image, 
            Vector2::new(((SCREEN_WIDTH/2) - (image.width() / 2)) as f32, ((SCREEN_HEIGHT/2) - (image.height() / 2)) as f32), 
            Color::WHITE
        );
        for ball in &balls {
            d.draw_circle_v(ball.position, ball.radius, ball.color);
        }
        // --- DRAW --- //
    }
}
