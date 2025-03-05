mod c8;
mod font;
mod gfxhandler;

use std::thread;
use std::time::{Duration, Instant};
use c8::C8;
use gfxhandler::GfxHandler;

const MICROSECONDS_PER_CYCLE: u64 = 2000;

fn main() {
    // setup graphics
    let mut gfxh: GfxHandler = GfxHandler::new().expect("Could not setup graphics");

    // setup input

    let mut chip_8: C8 = C8::new();
    
    // load game
    chip_8.read_program("bin/output.bin");
    chip_8.mem_dump();

    let cycle_duration = Duration::from_micros(MICROSECONDS_PER_CYCLE);
    // emulation loop
    loop {
        if chip_8.get_pc() >= 4096 { break; }

        let start_time = Instant::now();

        chip_8.emulate_cycle();

        // drawGraphics if draw flag is set
        if chip_8.get_draw() {
            gfxh.update_gfx(&chip_8.get_gfx());
        }

        // store key press states

        let elapsed = start_time.elapsed();
        if elapsed < cycle_duration {
            thread::sleep(cycle_duration - elapsed);
        }
    }

    // test
    loop {}
}
