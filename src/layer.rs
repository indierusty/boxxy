use macroquad::prelude::*;

pub struct Layer {
    pixels: Vec<Color>,
    preview_pixels: Vec<Option<Color>>,
    size: UVec2,
    transform: Affine2,
}

impl Layer {
    pub fn new(size: UVec2, position: Vec2) -> Self {
        let length = (size.x * size.y) as usize;

        let mut pixels = Vec::with_capacity(length);
        pixels.resize(length, BLACK);

        let mut preview_pixels = Vec::with_capacity(length);
        preview_pixels.resize(length, None);

        Self {
            pixels,
            preview_pixels,
            size,
            transform: Affine2::from_scale_angle_translation(vec2(10., 10.), 0., position),
        }
    }

    pub fn draw(&mut self) {
        let pixels: Vec<u8> = self
            .pixels
            .iter()
            .zip(self.preview_pixels.iter())
            .flat_map(|(a, b)| Into::<[u8; 4]>::into(b.unwrap_or(*a)))
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

    /// Sets pixel color at the given position. `pos` must be in the layer space coordinate.
    pub fn draw_pixel(&mut self, x: f32, y: f32, color: Color, preview: bool) {
        let (x, y) = (x.floor(), y.floor());
        let size = vec2(self.size.x as f32, self.size.y as f32);

        if x < 0. || x >= size.x || y < 0. || y >= size.y {
            return;
        }

        let index = (y * size.x) + x;
        let index = index as usize;

        if preview {
            self.preview_pixels[index] = Some(color);
        } else {
            self.pixels[index] = color;
        }
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color, preview: bool) {
        let mut xi = x;
        let mut yi = y;
        let ex = x + w;
        let ey = y + h;

        while xi <= ex {
            self.draw_pixel(xi, yi, color, preview);
            self.draw_pixel(xi, ey, color, preview);
            xi += 1.;
        }

        xi = x;
        yi = y;
        while yi <= ey {
            self.draw_pixel(xi, yi, color, preview);
            self.draw_pixel(ex, yi, color, preview);
            yi += 1.;
        }
    }

    pub fn clear_preview(&mut self) {
        for i in 0..self.preview_pixels.len() {
            self.preview_pixels[i] = None;
        }
    }
}

impl Layer {
    /// Transform a point in screen space coordinate to layer space coordinate.
    pub fn screen_to_local(&self, point: Vec2) -> Vec2 {
        self.transform.matrix2.inverse() * (point - self.transform.translation)
    }

    /// Adds translation to the layer.
    pub fn translate(&mut self, factor: Vec2) {
        self.transform.translation += factor;
    }

    /// Scale the layer.
    pub fn scale(&mut self, factor: Vec2) {
        self.transform.matrix2 *= Mat2::from_scale_angle(factor, 0.);
    }
}
