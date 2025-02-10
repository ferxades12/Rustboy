use std::time::Duration;

use crate::cpu::CPU;

mod cpu;
mod mmu;
mod op_codes;

const CYCLES_PER_FRAME: u32 = 70224;
const FRAME_TIME: Duration = Duration::from_micros((1_000_000.0 / 59.7) as u64);
const ROM_PATH: &str = r"rom\test\cpu_instrs\individual\11-op a,(hl).gb";

fn main() {
    let mut cpu = CPU::new();
    let mut last_frame_time = std::time::Instant::now();

    // Load the ROM into memory
    cpu.mmu.read_rom(ROM_PATH);
    // Set initial values for registers

    // Start the fetch-decode-execute cycle

    loop {
        //print!("cycles: {}\n", cpu.registers.A);
        let mut cycles_elapsed: u32 = 0;

        while cycles_elapsed < CYCLES_PER_FRAME {
            // log con el formato : 01 F: B0 B: 00 C: 13 D: 00 E: D8 H: 01 L: 4D SP: FFFE PC: 00:0101  (mem[pc] mem[pc+1] mem[pc+2] mem[pc+3])
            //print!("{:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: {:04X}  ({:02X} {:02X} {:02X} {:02X}) ", cpu.registers.A, cpu.registers.F, cpu.registers.B, cpu.registers.C, cpu.registers.D, cpu.registers.E, cpu.registers.H, cpu.registers.L, cpu.registers.SP, cpu.registers.PC, cpu.mmu.read_byte(cpu.registers.PC), cpu.mmu.read_byte(cpu.registers.PC + 1), cpu.mmu.read_byte(cpu.registers.PC + 2), cpu.mmu.read_byte(cpu.registers.PC + 3));
            cycles_elapsed += cpu.step() as u32;

            if cpu.halt_flag {
                break;
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
