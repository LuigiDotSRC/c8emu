use minifb::{Window, WindowOptions};

const PIXEL_SCALE: usize = 10;

pub struct GfxHandler {
    window: Window,
    buffer: Vec<u32>
}

impl GfxHandler {
    pub fn new() -> Option<Self> {
        let window = Window::new(
            "CHIP-8 Emulator",
            64 * PIXEL_SCALE,
            32 * PIXEL_SCALE,
            WindowOptions::default(),
        )
        .ok()?;

        Some(GfxHandler {
            window,
            buffer: vec![0x0000000000; (64 * PIXEL_SCALE) * (32 * PIXEL_SCALE)],
        })
    }

    pub fn update_gfx(&mut self, gfx: &[[u8; 64]; 32]) {
        for y in 0..32 {
            for x in 0..64 {
                let color = if gfx[y][x] > 0 { 0xFFFFFFFF } else { 0x00000000 };
                
                let start_x = x * PIXEL_SCALE;
                let start_y = y * PIXEL_SCALE;

                for sy in 0..PIXEL_SCALE {
                    for sx in 0..PIXEL_SCALE {
                        let index = (start_y + sy) * (64 * PIXEL_SCALE) + (start_x + sx);
                        self.buffer[index] = color;
                    }
                }
            }
        }

        self.window.update_with_buffer(&self.buffer, 64 * PIXEL_SCALE, 32 * PIXEL_SCALE).unwrap();
    }
}