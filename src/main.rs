use macroquad::prelude::*;

mod brush;
mod layer;

use brush::BrushTool;
use layer::Layer;

#[macroquad::main("Boxxy")]
async fn main() {
    let mut layer = Layer::new(uvec2(50, 50), vec2(100., 50.));
    let mut brush_tool = BrushTool::init();

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

        brush_tool.update(&mut layer);

        layer.translate(translation);
        layer.scale(scale);

        layer.draw();
        next_frame().await;
    }
}
