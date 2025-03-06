use minifb::{Window, WindowOptions, Key};
use std::collections::HashMap;

const PIXEL_SCALE: usize = 10;

pub struct GfxHandler {
    window: Window,
    buffer: Vec<u32>,
    keys: [u8; 16],
    keymap: HashMap<Key, usize>,
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

        let keys = [0u8; 16];

        let keymap: HashMap<Key, usize> = [
            (Key::Key1, 0x1), (Key::Key2, 0x2), (Key::Key3, 0x3), (Key::Key4, 0xC),
            (Key::Q, 0x4),    (Key::W, 0x5),    (Key::E, 0x6),    (Key::R, 0xD),
            (Key::A, 0x7),    (Key::S, 0x8),    (Key::D, 0x9),    (Key::F, 0xE),
            (Key::Z, 0xA),    (Key::X, 0x0),    (Key::C, 0xB),    (Key::V, 0xF),
        ]
        .iter()
        .cloned()
        .collect();

        Some(GfxHandler {
            window,
            buffer: vec![0x0000000000; (64 * PIXEL_SCALE) * (32 * PIXEL_SCALE)],
            keys,
            keymap,
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

    pub fn update_keys(&mut self) {
        let keys_pressed = self.window.get_keys();
    
        // reset keys
        self.keys = [0; 16];
    
        for key in keys_pressed {
            if let Some(&chip8_key) = self.keymap.get(&key) {
                self.keys[chip8_key] = 1;
            }
        }
    }

    pub fn get_keys(&self) -> [u8; 16] {
        self.keys
    }
}