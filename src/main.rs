mod c8;
mod font;

use c8::C8;

const OPCODES_PER_SECOND: usize = 500;

fn main() {
    // setup graphics
    // setup input

    let mut chip_8: C8 = C8::new();
    
    // load game
    chip_8.read_program("bin/output.bin");
    chip_8.mem_dump();
    // emulation loop

        // emulate cycle
        chip_8.emulate_cycle();
        // drawGraphics if draw fag is set

        // store key press states
}
