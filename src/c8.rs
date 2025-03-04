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

    pub fn read_program() {
        // read a binary file

        // load it into memory starting at 0x200
    }

    pub fn emulate_cycle(&mut self) {
        // test
        self.memory[0] = 0xA2;
        self.memory[1] = 0xF0;
        
        self.opcode = (self.memory[0] as u16) << 8 | (self.memory[1] as u16);

        self.i_reg = self.opcode & 0x0FFF;

        self.pc += 2;
        println!("{:X}", self.opcode);
        println!("{:X}", self.i_reg);
    }
}