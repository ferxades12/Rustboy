use std::time::Duration;

use mmu::MMU;

use crate::cpu::CPU;

mod cpu;
pub mod gpu;
pub mod mmu;
mod op_codes;
// T-cycles = Clock cycles. 1 M-cycle = 4 T-cycles
const CYCLES_PER_FRAME: u32 = 70224 / 4; // M-cycles.
const FRAME_TIME: Duration = Duration::from_micros((1_000_000.0 / 59.7) as u64);
const ROM_PATH: &str = r"rom\test\instr_timing\instr_timing.gb";

fn main() {
    let mut mmu = MMU::new();
    let mut cpu = CPU::new();
    let mut gpu = gpu::Screen::new();
    let mut gpu_dots = 0;
    let mut last_frame_time = std::time::Instant::now();

    // Load the ROM into memory
    mmu.read_rom(ROM_PATH);

    // Start the fetch-decode-execute cycle
    loop {
        //print!("cycles: {}\n", cpu.registers.A);
        let mut cycles_elapsed: u32 = 0;

        while cycles_elapsed < CYCLES_PER_FRAME {
            let cycles = cpu.step(&mut mmu) as u32;
            cycles_elapsed += cycles;

            gpu_dots -= cycles * 4;
            if gpu_dots <= 0 {
                gpu_dots += gpu.step(&mut mmu) as u32;
            }
        }

        // Sincronizar tiempo
        let elapsed_time = last_frame_time.elapsed();
        if elapsed_time < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - elapsed_time);
        }

        last_frame_time = std::time::Instant::now();
    }
}
