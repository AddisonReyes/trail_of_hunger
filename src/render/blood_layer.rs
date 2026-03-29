use macroquad::prelude::*;

pub struct BloodLayer {
    rt: RenderTarget,
    cam: Camera2D,
    w: u32,
    h: u32,
}

impl BloodLayer {
    pub fn new(playfield_w: u32, playfield_h: u32) -> Self {
        let (w, h) = (playfield_w.max(1), playfield_h.max(1));
        let rt = render_target(w, h);

        let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, w as f32, h as f32));
        cam.render_target = Some(rt.clone());

        let mut layer = Self { rt, cam, w, h };
        layer.reset();
        layer
    }

    pub fn ensure_size(&mut self, playfield_w: u32, playfield_h: u32) {
        let (w, h) = (playfield_w.max(1), playfield_h.max(1));
        if w == self.w && h == self.h {
            return;
        }

        self.w = w;
        self.h = h;
        self.rt = render_target(w, h);
        self.cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, w as f32, h as f32));
        self.cam.render_target = Some(self.rt.clone());
        self.reset();
    }

    pub fn reset(&mut self) {
        set_camera(&self.cam);
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));
        set_default_camera();
    }

    pub fn paint_drop(&mut self, pos_world: Vec2, radius: f32, color: Color) {
        set_camera(&self.cam);
        draw_circle(pos_world.x, pos_world.y, radius.max(0.0), color);
        set_default_camera();
    }

    pub fn draw(&self, offset_y: f32) {
        draw_texture_ex(
            &self.rt.texture,
            0.0,
            offset_y,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );
    }
}
