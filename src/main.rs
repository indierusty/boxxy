use macroquad::prelude::*;

#[macroquad::main("Boxxy")]
async fn main() {
    let mut strokes = Vec::new();

    loop {
        clear_background(WHITE);
        if is_mouse_button_down(MouseButton::Left) {
            strokes.push(vec2(mouse_position().0, mouse_position().1));
        }
        for s in &strokes {
            draw_circle(s.x, s.y, 5., DARKGRAY);
        }
        next_frame().await;
    }
}
