use crate::{Operand, Register16, Register8, RegisterPair, CPU};


fn execute_opcode(cpu: &mut CPU){
    let opcode = cpu.fetch_byte();
    match opcode {
        0x00 => {},
        0x01 => { // LD BC, u16
            let value = cpu.fetch_word();
            cpu.LD(RegisterPair::BC, value);
        },
        0x02 => { // LD (BC), A
            cpu.LD::<u8>(RegisterPair::BC, Register8::A);
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
            cpu.LD(word, Register16::SP);
        },
        0x09 => { // ADD HL, BC
            let result = cpu.ADD(cpu.get_register_pair(RegisterPair::HL), cpu.get_register_pair(RegisterPair::BC));
            cpu.set_register_pair(RegisterPair::HL, result);
        },
        0x0A => { // LD A, (BC)
            cpu.LD::<u8>(Register8::A, RegisterPair::BC);
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
        0x16 => { // LD D, u8
            let value = cpu.fetch_byte();
            cpu.LD(Register8::D, value);
        },
        0x17 => { // RLA
            cpu.RLA();
        },
        0x18 => { // JR i8
            let value = cpu.fetch_byte();
            //cpu.JR(value);
        },
        0x19 => { // ADD HL, DE
            let result = cpu.ADD(cpu.get_register_pair(RegisterPair::HL), cpu.get_register_pair(RegisterPair::DE));
            cpu.set_register_pair(RegisterPair::HL, result);
        },
        0x1A => { // LD A, (DE)
            cpu.LD::<u8>(Register8::A, RegisterPair::DE);
        },
        0x1B => { // DEC DE
            cpu.DEC(RegisterPair::DE);
        },
        0x1C => { // INC E
            cpu.INC(Register8::E);
        },
        0x1D => { // DEC E
            cpu.DEC(Register8::E);
        },
        0x1E => { // LD E, u8
            let value = cpu.fetch_byte();
            cpu.LD(Register8::E, value);
        },
        0x1F => { // RRA
            //cpu.RRA();
        },
        0x20 => { // JR NZ, i8
            let value = cpu.fetch_byte();
            //cpu.JR_NZ(value);
        },
        0x21 => { // LD HL, u16
            let value = cpu.fetch_word();
            cpu.LD(RegisterPair::HL, value);
        },
        0x22 => { // LD (HL+), A
            cpu.LD::<u8>(RegisterPair::HL, Register8::A);
            cpu.INC(RegisterPair::HL);
        },
        0x23 => { // INC HL
            cpu.INC(RegisterPair::HL);
        },
        0x24 => { // INC H
            cpu.INC(Register8::H);
        },
        0x25 => { // DEC H
            cpu.DEC(Register8::H);
        },
        0x26 => { // LD H, u8
            let value = cpu.fetch_byte();
            cpu.LD(Register8::H, value);
        },
        0x27 => { // DAA
            //cpu.DAA();
        },
        0x28 => { // JR Z, i8
            let value = cpu.fetch_byte();
            //cpu.JR_Z(value);
        },
        0x29 => { // ADD HL, HL
            let result = cpu.ADD(cpu.get_register_pair(RegisterPair::HL), cpu.get_register_pair(RegisterPair::HL));
            cpu.set_register_pair(RegisterPair::HL, result);
        },
        0x2A => { // LD A, (HL+)
            cpu.LD::<u8>(Register8::A, RegisterPair::HL);
            cpu.INC(RegisterPair::HL);
        },
        0x2B => { // DEC HL
            cpu.DEC(RegisterPair::HL);
        },
        0x2C => { // INC L
            cpu.INC(Register8::L);
        },
        0x2D => { // DEC L
            cpu.DEC(Register8::L);
        },
        0x2E => { // LD L, u8
            let value = cpu.fetch_byte();
            cpu.LD(Register8::L, value);
        },
        0x2F => { // CPL
            //cpu.CPL();
        },
        0x30 => { // JR NC, i8
            let value = cpu.fetch_byte();
            //cpu.JR_NC(value);
        },
        0x31 => { // LD SP, u16
            let value = cpu.fetch_word();
            cpu.LD(Register16::SP, value);
        },
        _ => panic!("Unknown opcode: 0x{:X}", opcode),
    }
}