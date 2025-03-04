pub struct C8 {
    memory: [u8; 4096],
    v_regs: [u8; 16], // V0 to VF
    i_reg: u8,
    pc: u8, 
    opcode: [u8; 2],

    gfx: [[bool; 64]; 32],

    delay_timer: u8,
    sound_timer: u8,

    stack: [u8; 16],// 16 levels of nesting
    sp: u8,
    
    key: [bool; 16], // hex keyboard
}

impl C8 {
    pub fn new() -> Self {
        Self {
            memory: [0x00; 4096],
            v_regs: [0x00; 16],
            i_reg: 0x00,
            pc: 0x00,
            opcode: [0x00; 2],
            gfx: [[false; 64]; 32],
            delay_timer: 0x00,
            sound_timer: 0x00,
            stack: [0x00; 16],
            sp: 0x00,
            key: [false; 16]
        }
    }
}