use macroquad::prelude::*;

mod brush;
mod layer;
mod rect;

use brush::BrushTool;
use layer::Layer;
use rect::RectTool;

pub enum Tool {
    Rect,
    Brush,
}

#[macroquad::main("Boxxy")]
async fn main() {
    let mut layer = Layer::new(uvec2(300, 300), vec2(100., 50.));
    let mut active_tool = Tool::Brush;

    let mut brush_tool = BrushTool::init();
    let mut rect_tool = RectTool::new();

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

        if is_key_pressed(KeyCode::B) {
            active_tool = Tool::Brush;
        } else if is_key_pressed(KeyCode::R) {
            active_tool = Tool::Rect;
        }

        match active_tool {
            Tool::Rect => rect_tool.update(&mut layer),
            Tool::Brush => brush_tool.update(&mut layer),
        }

        layer.translate(translation);
        layer.scale(scale);

        layer.draw();
        next_frame().await;
    }
}
