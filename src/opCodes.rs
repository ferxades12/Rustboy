use crate::{RegisterPair, CPU};


fn execute_opcode(cpu: &mut CPU){
    let opcode = cpu.fetch_byte();
    match opcode {
        0x00 => {},
        0x01 => { // LD BC, u16
            let value = cpu.fetch_word();
            cpu.LD(RegisterPair::BC, value);
        },
        _ => panic!("Unknown opcode: 0x{:X}", opcode),
    }
}