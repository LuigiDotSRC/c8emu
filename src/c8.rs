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
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc as usize) + 1] as u16);

        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((self.opcode & 0x00F0) >> 4) as u8;
        let nn: u8 = (self.opcode & 0x00FF) as u8;

        match self.opcode & 0xF000 {
            0x0000 => {
                match self.opcode & 0x0FFF {
                    // FLOW: return from subroutine
                    0x00EE => {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                    }

                    _ => self.unknown_opcode(),
                }
            }
            // FLOW: jump to address NNN
            0x1000 => self.pc = self.opcode & 0x0FFF,

            // FLOW: call subroutine at NNN
            0x2000 => {
                self.stack[self.sp as usize] = self.pc + 2; // 2 byte offset
                self.sp += 1;

                self.pc = self.opcode & 0x0FFF;
            }

            // COND: skip next instruction if VX = NN
            0x3000 => {
                if nn == self.v_regs[x as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            // COND: skip next instruction if VX != NN
            0x4000 => {
                if nn != self.v_regs[x as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            // COND: skip next instruction if VX = VY
            0x5000 => {
                if self.v_regs[x as usize] == self.v_regs[y as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            // CONST: set VX to NN
            0x6000 => {
                self.v_regs[x as usize] = nn;
                self.pc += 2;
            }

            // CONST: add NN to VX
            0x7000 => {
                self.v_regs[x as usize] += nn;
                self.pc += 2;
            }

            0x8000 => {
                match self.opcode & 0x000F {
                    // ASSIGN: set VX to VY
                    0x0000 => {
                        self.v_regs[x as usize] = self.v_regs[y as usize];
                        self.pc += 2;
                    }

                    // BITOP: VX = VX or VY
                    0x0001 => {
                        self.v_regs[x as usize] |= self.v_regs[y as usize];
                        self.pc += 2;
                    }

                    // BITOP: VX = VX and VY
                    0x0002 => {
                        self.v_regs[x as usize] &= self.v_regs[y as usize];
                        self.pc += 2;
                    }

                    // BITOP: VX = VX xor VY
                    0x0003 => {
                        self.v_regs[x as usize] ^= self.v_regs[y as usize];
                        self.pc += 2;
                    }

                    // BITOP: shift VX to the right by 1, set VF to prev LSB  
                    0x0006 => {
                        self.v_regs[0xF] = self.v_regs[x as usize] & 0x01;
                        self.v_regs[x as usize] >>= 1;
                        self.pc += 2;
                    }

                    // BITOP: shift VX to left by 1, set VF to prev MSB
                    0x000E => {
                        self.v_regs[0xF] = (self.v_regs[x as usize] & 0x80) >> 7;
                        self.v_regs[x as usize] <<= 1;
                        self.pc += 2;
                    }

                    _ => self.unknown_opcode(),
                }
            }

            // MEM: sets i register to address NNN
            0xA000 => {
                self.i_reg = self.opcode & 0x0FFF;
                self.pc += 2;
            }

            // FLOW: jumps to address NNN + V0
            0xB000 => self.pc = (self.opcode & 0x0FFF) + self.v_regs[0] as u16, 

            0xF000 => {                
                match self.opcode & 0x00FF {
                    // MEM: add VX to I
                    0x001E => {
                        self.i_reg += self.v_regs[x as usize] as u16;
                        self.pc += 2;
                    }

                    // MEM: set I to locaiton of sprite fontset for char (lowest nibble) in VX
                    0x0029 => {
                        let c: u8 = self.v_regs[x as usize] & 0x0F;
                        self.i_reg = 0x0000 + (c as u16 * 5);
                        self.pc += 2;
                    }

                    // MEM: store V0 to VX in memory starting at I
                    0x0055 => {
                        for i in 0..=x {
                            self.memory[self.i_reg as usize + i as usize] = self.v_regs[i as usize];
                        }
                        self.pc += 2;
                    }

                    // MEM: load V0 to VX from memory starting at I
                    0x0065 => {
                        for i in 0..=x {
                            self.v_regs[i as usize] = self.memory[self.i_reg as usize + i as usize];
                        }
                        self.pc += 2;
                    }

                    _ => self.unknown_opcode(),
                }
            }

            _ => self.unknown_opcode(),
        }

        // update timers
        if self.delay_timer > 0 { self.delay_timer -= 1; }
        if self.sound_timer > 0 {
            println!("BEEP");
            self.sound_timer -= 1;
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    fn unknown_opcode(&mut self) {
        println!("{:04X}: Unknown opcode {:04X}", self.pc, self.opcode);
        self.pc += 2;
    }
}