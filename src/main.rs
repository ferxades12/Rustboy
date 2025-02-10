use crate::cpu::CPU;

mod cpu;
mod mmu;
mod op_codes;

fn main() {
    let mut cpu = CPU::new();

    // Load the ROM into memory

    // Set initial values for registers

    // Start the fetch-decode-execute cycle
    loop {
        if cpu.halt_flag {
            break;
        }

        cpu.step();
    }
}
