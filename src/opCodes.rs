
use crate::CPU;


fn execute_opcode(cpu: &mut CPU){
    let opcode = cpu.fetch_byte();
    match opcode {
        0x00 => {},
        0x01 => { // LD BC, u16
            let value = cpu.fetch_word();
            cpu.set_bc(value);
        },
        0x02 => { // LD (BC), A
            cpu.memory[cpu.get_bc() as usize] = cpu.registers.A;
        },
        0x03 => { // INC BC
            let value = cpu.get_bc().wrapping_add(1);
            cpu.set_bc(value);
        },
        0x04 => { // INC B
            cpu.registers.B = cpu.INC(cpu.registers.B);
        },
        0x05 => { // DEC B
            cpu.registers.B = cpu.DEC(cpu.registers.B);
        },
        0x06 => { // LD B, u8
            let value = cpu.fetch_byte();
            cpu.registers.B = value;
        },
        0x07 => { // RLCA
            cpu.RLCA();
        },
        0x08 => { // LD (u16), SP
            let word = cpu.fetch_word();
            let low = word as u8;
            let high = (word >> 8) as u8;
            cpu.memory[word as usize] = low;
            cpu.memory[(word + 1) as usize] = high;
        },
        0x09 => { // ADD HL, BC
            let zf = cpu.get_ZF();
            let result = cpu.ADD(cpu.get_hl(), cpu.get_bc());
            cpu.set_hl(result);
            cpu.set_ZF(zf);
        },
        0x0A => { // LD A, (BC)
            cpu.registers.A = cpu.memory[cpu.get_bc() as usize];
        },
        0x0B => { // DEC BC
            let value = cpu.get_bc().wrapping_sub(1);
            cpu.set_bc(value);
        },
        0x0C => { // INC C
            cpu.registers.C = cpu.INC(cpu.registers.C);
        },
        0x0D => { // DEC C
            cpu.registers.C = cpu.DEC(cpu.registers.C);
        },
        0x0E => { // LD C, u8
            let value = cpu.fetch_byte();
            cpu.registers.C = value;
        },
        0x0F => { // RRCA
            //cpu.RRCA();
        },
        0x10 => { // STOP
            //cpu.STOP();
        },
        0x11 => { // LD DE, u16
            let value = cpu.fetch_word();
            cpu.set_de(value);
        },
        0x12 => { // LD (DE), A
            cpu.memory[cpu.get_de() as usize] = cpu.registers.A;
        },
        0x13 => { // INC DE
            let value = cpu.get_de().wrapping_add(1);
            cpu.set_de(value);
        },
        0x14 => { // INC D
            cpu.registers.D = cpu.INC(cpu.registers.D);
        },
        0x15 => { // DEC D
            cpu.registers.D = cpu.DEC(cpu.registers.D);
        },
        0x16 => { // LD D, u8
            let value = cpu.fetch_byte();
            cpu.registers.D = value;
        },
        0x17 => { // RLA
            cpu.RLA();
        },
        0x18 => { // JR i8
            let value = cpu.fetch_byte();
            //cpu.JR(value);
        },
        0x19 => { // ADD HL, DE
            let zf = cpu.get_ZF();
            let result = cpu.ADD(cpu.get_hl(), cpu.get_de());
            cpu.set_hl(result);
            cpu.set_ZF(zf);
        },
        0x1A => { // LD A, (DE)
            cpu.registers.A = cpu.memory[cpu.get_de() as usize];
        },
        0x1B => { // DEC DE
            let value = cpu.get_de().wrapping_sub(1);
            cpu.set_de(value);
        },
        0x1C => { // INC E
            cpu.registers.E = cpu.INC(cpu.registers.E);
        },
        0x1D => { // DEC E
            cpu.registers.E = cpu.DEC(cpu.registers.E);
        },
        0x1E => { // LD E, u8
            let value = cpu.fetch_byte();
            cpu.registers.E = value;
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
            cpu.set_hl(value);
        },
        0x22 => { // LD (HL+), A
            let hl = cpu.get_hl();
            cpu.memory[hl as usize] = cpu.registers.A;
            cpu.set_hl(hl.wrapping_add(1));
        },
        0x23 => { // INC HL
            let value = cpu.get_hl().wrapping_add(1);
            cpu.set_hl(value);
        },
        0x24 => { // INC H
            cpu.registers.H = cpu.INC(cpu.registers.H);
        },
        0x25 => { // DEC H
            cpu.registers.H = cpu.DEC(cpu.registers.H);
        },
        0x26 => { // LD H, u8
            let value = cpu.fetch_byte();
            cpu.registers.H = value;
        },
        0x27 => { // DAA
            //cpu.DAA();
        },
        0x28 => { // JR Z, i8
            let value = cpu.fetch_byte();
            //cpu.JR_Z(value);
        },
        0x29 => { // ADD HL, HL
            let zf = cpu.get_ZF();
            let result = cpu.ADD(cpu.get_hl(), cpu.get_hl());
            cpu.set_hl(result);
            cpu.set_ZF(zf);
        },
        0x2A => { // LD A, (HL+)
            let hl = cpu.get_hl();
            cpu.registers.A = cpu.memory[hl as usize];
            cpu.set_hl(hl.wrapping_add(1));
        },
        0x2B => { // DEC HL
            let value = cpu.get_hl().wrapping_sub(1);
            cpu.set_hl(value);
        },
        0x2C => { // INC L
            cpu.registers.L = cpu.INC(cpu.registers.L);
        },
        0x2D => { // DEC L
            cpu.registers.L = cpu.DEC(cpu.registers.L);
        },
        0x2E => { // LD L, u8
            let value = cpu.fetch_byte();
            cpu.registers.L = value;
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
            cpu.registers.SP = value;
        },
        0x32 => { // LD (HL-), A
            let hl = cpu.get_hl();
            cpu.memory[hl as usize] = cpu.registers.A;
            cpu.set_hl(hl.wrapping_sub(1));
        },
        0x33 => { // INC SP
            let value = cpu.registers.SP.wrapping_add(1);
            cpu.registers.SP = value;
        },
        0x34 => { // INC (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.INC(value);
            cpu.memory[cpu.get_hl() as usize] = result;
        },
        0x35 => { // DEC (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.DEC(value);
            cpu.memory[cpu.get_hl() as usize] = result;
        },
        0x36 => { // LD (HL), u8
            let value = cpu.fetch_byte();
            cpu.memory[cpu.get_hl() as usize] = value;
        },
        0x37 => { // SCF
            //cpu.SCF();
        },
        0x38 => { // JR C, i8
            let value = cpu.fetch_byte();
            //cpu.JR_C(value);
        },
        0x39 => { // ADD HL, SP
            let zf = cpu.get_ZF();
            let result = cpu.ADD(cpu.get_hl(), cpu.registers.SP);
            cpu.set_hl(result);
            cpu.set_ZF(zf);
        },
        _ => panic!("Unknown opcode: 0x{:X}", opcode),
    }
}