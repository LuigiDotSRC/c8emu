mod c8;
mod font;
mod gfxhandler;

use std::thread;
use std::time::{Duration, Instant};
use std::env;
use c8::C8;
use gfxhandler::GfxHandler;

const MICROSECONDS_PER_CYCLE: u64 = 2000;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Expected file path");
        return;
    }
    let file_path = &args[1];

    let mut gfxh: GfxHandler = GfxHandler::new().expect("Could not setup graphics");
    let mut chip_8: C8 = C8::new();
    
    chip_8.read_program(file_path);
    chip_8.mem_dump();

    let cycle_duration = Duration::from_micros(MICROSECONDS_PER_CYCLE);

    // emulation loop
    loop {
        if chip_8.get_pc() >= 4096 { break; }

        let start_time = Instant::now();

        gfxh.update_keys();
        chip_8.set_keys(gfxh.get_keys());

        chip_8.emulate_cycle();

        // drawGraphics if draw flag is set
        if chip_8.get_draw() {
            gfxh.update_gfx(&chip_8.get_gfx());
        }

        let elapsed = start_time.elapsed();
        if elapsed < cycle_duration {
            thread::sleep(cycle_duration - elapsed);
        }
    }
}
