use macroquad::prelude::*;

pub struct Layer {
    pixels: Vec<Color>,
    size: UVec2,
    transform: Affine2,
}

impl Layer {
    pub fn new(size: UVec2, position: Vec2) -> Self {
        let length = (size.x * size.y) as usize;
        let mut pixels = Vec::with_capacity(length);
        pixels.resize(length, BLACK);
        Self {
            pixels,
            size,
            transform: Affine2::from_scale_angle_translation(vec2(10., 10.), 0., position),
        }
    }

    pub fn draw(&mut self) {
        let pixels: Vec<u8> = self
            .pixels
            .iter()
            .flat_map(|c| [c.r, c.g, c.b, c.a])
            .map(|v| (v * 255.) as u8)
            .collect();

        let texture = Texture2D::from_rgba8(self.size.x as u16, self.size.y as u16, &pixels);
        texture.set_filter(FilterMode::Nearest);

        let x = self.transform.translation.x;
        let y = self.transform.translation.y;
        let size = self.transform.matrix2 * vec2(self.size.x as f32, self.size.y as f32);

        draw_texture_ex(
            &texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
    }

    pub fn pixel(&mut self, pos: Vec2) {
        let index = (pos.y as u32 * self.size.x) + pos.x as u32;
        let index = index as usize;
        if index < self.pixels.len() {
            self.pixels[index] = BLUE;
        }
    }
}

impl Layer {
    pub fn screen_to_local(&self, point: Vec2) -> Vec2 {
        self.transform.matrix2.inverse() * (point - self.transform.translation)
    }

    pub fn translate(&mut self, factor: Vec2) {
        self.transform.translation += factor;
    }

    pub fn scale(&mut self, factor: Vec2) {
        self.transform.matrix2 *= Mat2::from_scale_angle(factor, 0.);
    }
}
