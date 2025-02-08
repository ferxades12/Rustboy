use crate::cpu::CPU;
use crate::op_codes::execute_opcode;

mod cpu;
mod op_codes;


fn main() {
    let mut cpu = CPU::new();

    // Load the ROM into memory

    // Set the program counter to the start of the ROM

    // Start the fetch-decode-execute cycle
    loop{
        if cpu.ei_flag{
            cpu.ei_flag = false;
            cpu.registers.IME = true;
        }

        execute_opcode(&mut cpu);

        if cpu.halt_flag {break;}
    }

}
