use crate::cpu::CPU;
use crate::op_codes::execute_opcode;

mod cpu;
mod op_codes;


fn main() {
    let mut cpu = CPU::new();

    execute_opcode(&mut cpu);
    
}
