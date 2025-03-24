use macroquad::prelude::*;

use crate::layer::Layer;

pub struct BrushTool;

impl BrushTool {
    pub fn init() -> Self {
        Self {}
    }

    pub fn update(&mut self, layer: &mut Layer) {
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_position = vec2(mouse_position().0, mouse_position().1);
            let point = layer.screen_to_local(mouse_position);
            layer.draw_pixel(point, RED, false);
        }
    }
}
