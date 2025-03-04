use std::fs::File;
use std::io::{Read};
use crate::font;

pub struct C8 {
    memory: [u8; 4096],
    v_regs: [u8; 16], // V0 to VF
    i_reg: u16,
    pc: u16, 
    opcode: u16,

    gfx: [[u8; 64]; 32],

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16],// 16 levels of nesting
    sp: u16,
    
    key: [u8; 16], // hex keyboard
}

impl C8 {
    pub fn new() -> Self {
        let mut instance = Self {
            memory: [0x00; 4096],
            v_regs: [0x00; 16],
            i_reg: 0x0000,
            pc: 0x200, // programs expected to start at 0x200
            opcode: 0x0000,
            gfx: [[0x00; 64]; 32],
            delay_timer: 0x00,
            sound_timer: 0x00,
            stack: [0x0000; 16],
            sp: 0x0000,
            key: [0x00; 16]
        };

        // load fontset
        for i in 0..79 {
            instance.memory[i] = font::CHIP8_FONTSET[i];
        }

        instance
    }

    pub fn read_program(&mut self, file_path: &str) {
        // load a binary file into memory starting at 0x200
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut buffer: [u8; 1024] = [0; 1024];

        let mut mp = 0x200;
        while let Ok(n) = file.read(&mut buffer) {
            if n == 0 { break; } // EOF

            for i in 0..n { self.memory[mp + i] = buffer[i]; }
            mp += n;

            println!("Read {} bytes", n);
        }
    }

    pub fn mem_dump(&self) {
        for i in 0..(self.memory.len() / 16) {
            print!("{:04X}: ", i * 16); 
            
            // 16 bytes per row
            for j in 0..16 {
                print!("{:02X} ", self.memory[i * 16 + j]);
            }
            
            println!();
        }
    }

    pub fn emulate_cycle(&mut self) {
        // test
        self.memory[0x200] = 0xA2;
        self.memory[0x201] = 0xF0;
        
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc as usize) + 1] as u16);

        self.i_reg = self.opcode & 0x0FFF;

        self.pc += 2;
        println!("{:X}", self.opcode);
        println!("{:X}", self.i_reg);
    }
}