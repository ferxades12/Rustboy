use crate::{Register16, Register8, RegisterPair, CPU};


fn execute_opcode(cpu: &mut CPU){
    let opcode = cpu.fetch_byte();
    match opcode {
        0x00 => {},
        0x01 => { // LD BC, u16
            let value = cpu.fetch_word();
            cpu.LD(RegisterPair::BC, value);
        },
        0x02 => { // LD (BC), A
            cpu.LD(RegisterPair::BC, cpu.A);
        },
        0x03 => { // INC BC
            cpu.INC(RegisterPair::BC);
        },
        0x04 => { // INC B
            cpu.INC(Register8::B);
        },
        0x05 => { // DEC B
            cpu.DEC(Register8::B);
        },
        0x06 => { // LD B, u8
            let value = cpu.fetch_byte();
            cpu.LD(Register8::B, value);
        },
        0x07 => { // RLCA
            cpu.RLCA();
        },
        0x08 => { // LD (u16), SP
            let word = cpu.fetch_word();
            cpu.LD(word, cpu.SP);
        },
        0x09 => { // ADD HL, BC
            let result = cpu.ADD(cpu.get_register_pair(RegisterPair::HL), cpu.get_register_pair(RegisterPair::BC));
            cpu.set_register_pair(RegisterPair::HL, result);
        },
        0x0A => { // LD A, (BC)
            cpu.LD(Register8::A, RegisterPair::BC);
        },
        0x0B => { // DEC BC
            cpu.DEC(RegisterPair::BC);
        },
        0x0C => { // INC C
            cpu.INC(Register8::C);
        },
        0x0D => { // DEC C
            cpu.DEC(Register8::C);
        },
        0x0E => { // LD C, u8
            let value = cpu.fetch_byte();
            cpu.LD(Register8::C, value);
        },
        0x0F => { // RRCA
            //cpu.RRCA();
        },
        0x10 => { // STOP
            //cpu.STOP();
        },
        0x11 => { // LD DE, u16
            let value = cpu.fetch_word();
            cpu.LD(RegisterPair::DE, value);
        },
        0x12 => { // LD (DE), A
            cpu.LD::<u8>(RegisterPair::DE, cpu.A);
        },
        0x13 => { // INC DE
            cpu.INC(RegisterPair::DE);
        },
        0x14 => { // INC D
            cpu.INC(Register8::D);
        },
        0x15 => { // DEC D
            cpu.DEC(Register8::D);
        },
        _ => panic!("Unknown opcode: 0x{:X}", opcode),
    }
}