use macroquad::prelude::*;

use crate::layer::Layer;

enum RectToolState {
    Ready,
    Drawing,
}

pub struct RectTool {
    state: RectToolState,
    start_pos: Option<Vec2>,
    end_pos: Option<Vec2>,
}

impl RectTool {
    pub fn new() -> Self {
        Self {
            state: RectToolState::Ready,
            start_pos: None,
            end_pos: None,
        }
    }

    pub fn update(&mut self, layer: &mut Layer) {
        let state = match self.state {
            RectToolState::Ready => {
                self.start_pos = None;
                self.end_pos = None;

                if is_mouse_button_down(MouseButton::Left) {
                    let (x, y) = mouse_position();
                    self.start_pos = Some(vec2(x, y));

                    RectToolState::Drawing
                } else {
                    RectToolState::Ready
                }
            }
            RectToolState::Drawing => {
                if is_mouse_button_down(MouseButton::Left) {
                    let (x, y) = mouse_position();
                    self.end_pos = Some(vec2(x, y));

                    let start = self.start_pos.unwrap();
                    let end = self.end_pos.unwrap();
                    draw_rect(start, end, RED, true, layer);

                    RectToolState::Drawing
                } else {
                    let start = self.start_pos.unwrap();
                    let end = self.end_pos.unwrap();
                    draw_rect(start, end, RED, false, layer);

                    RectToolState::Ready
                }
            }
        };
        self.state = state;
    }
}

fn draw_rect(start: Vec2, end: Vec2, color: Color, preview: bool, layer: &mut Layer) {
    // Transform the positions from screen space to layer space.
    let start = layer.screen_to_local(start);
    let end = layer.screen_to_local(end);

    let (x, y) = (start.x.min(end.x), start.y.min(end.y));
    let (end_x, end_y) = (start.x.max(end.x), start.y.max(end.y));

    let w = end_x - x;
    let h = end_y - y;

    layer.clear_preview();
    layer.draw_rect(x, y, w, h, color, preview);
}
