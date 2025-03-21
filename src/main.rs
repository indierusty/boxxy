use macroquad::prelude::*;

mod layer;
use layer::Layer;

#[macroquad::main("Boxxy")]
async fn main() {
    let mut layer = Layer::new(uvec2(50, 50), vec2(100., 50.));

    loop {
        clear_background(WHITE);

        let mut scale = Vec2::ONE;
        if is_key_down(KeyCode::K) {
            scale = vec2(1.01, 1.01);
        } else if is_key_down(KeyCode::J) {
            scale = vec2(0.99, 0.99);
        }

        let mut translation = Vec2::ZERO;
        if is_key_down(KeyCode::Left) {
            translation.x = -5.;
        } else if is_key_down(KeyCode::Right) {
            translation.x = 5.;
        } else if is_key_down(KeyCode::Up) {
            translation.y = -5.;
        } else if is_key_down(KeyCode::Down) {
            translation.y = 5.;
        };

        layer.translate(translation);
        layer.scale(scale);

        if is_mouse_button_down(MouseButton::Left) {
            let point = layer.screen_to_local(vec2(mouse_position().0, mouse_position().1));
            layer.pixel(point);
        }
        layer.draw();
        next_frame().await;
    }
}
