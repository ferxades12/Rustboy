
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
        0x3A => { // LD A, (HL-)
            let hl = cpu.get_hl();
            cpu.registers.A = cpu.memory[hl as usize];
            cpu.set_hl(hl.wrapping_sub(1));
        },
        0x3B => { // DEC SP
            let value = cpu.registers.SP.wrapping_sub(1);
            cpu.registers.SP = value;
        },
        0x3C => { // INC A
            cpu.registers.A = cpu.INC(cpu.registers.A);
        },
        0x3D => { // DEC A
            cpu.registers.A = cpu.DEC(cpu.registers.A);
        },
        0x3E => { // LD A, u8
            let value = cpu.fetch_byte();
            cpu.registers.A = value;
        },
        0x3F => { // CCF
            //cpu.CCF();
        },
        0x40 => { // LD B, B
            cpu.registers.B = cpu.registers.B;
        },
        0x41 => { // LD B, C
            cpu.registers.B = cpu.registers.C;
        },
        0x42 => { // LD B, D
            cpu.registers.B = cpu.registers.D;
        },
        0x43 => { // LD B, E
            cpu.registers.B = cpu.registers.E;
        },
        0x44 => { // LD B, H
            cpu.registers.B = cpu.registers.H;
        },
        0x45 => { // LD B, L
            cpu.registers.B = cpu.registers.L;
        },
        0x46 => { // LD B, (HL)
            cpu.registers.B = cpu.memory[cpu.get_hl() as usize];
        },
        0x47 => { // LD B, A
            cpu.registers.B = cpu.registers.A;
        },
        0x48 => { // LD C, B
            cpu.registers.C = cpu.registers.B;
        },
        0x49 => { // LD C, C
            cpu.registers.C = cpu.registers.C;
        },
        0x4A => { // LD C, D
            cpu.registers.C = cpu.registers.D;
        },
        0x4B => { // LD C, E
            cpu.registers.C = cpu.registers.E;
        },
        0x4C => { // LD C, H
            cpu.registers.C = cpu.registers.H;
        },
        0x4D => { // LD C, L
            cpu.registers.C = cpu.registers.L;
        },
        0x4E => { // LD C, (HL)
            cpu.registers.C = cpu.memory[cpu.get_hl() as usize];
        },
        0x4F => { // LD C, A
            cpu.registers.C = cpu.registers.A;
        },
        0x50 => { // LD D, B
            cpu.registers.D = cpu.registers.B;
        },
        0x51 => { // LD D, C
            cpu.registers.D = cpu.registers.C;
        },
        0x52 => { // LD D, D
            cpu.registers.D = cpu.registers.D;
        },
        0x53 => { // LD D, E
            cpu.registers.D = cpu.registers.E;
        },
        0x54 => { // LD D, H
            cpu.registers.D = cpu.registers.H;
        },
        0x55 => { // LD D, L
            cpu.registers.D = cpu.registers.L;
        },
        0x56 => { // LD D, (HL)
            cpu.registers.D = cpu.memory[cpu.get_hl() as usize];
        },
        0x57 => { // LD D, A
            cpu.registers.D = cpu.registers.A;
        },
        0x58 => { // LD E, B
            cpu.registers.E = cpu.registers.B;
        },
        0x59 => { // LD E, C
            cpu.registers.E = cpu.registers.C;
        },
        0x5A => { // LD E, D
            cpu.registers.E = cpu.registers.D;
        },
        0x5B => { // LD E, E
            cpu.registers.E = cpu.registers.E;
        },
        0x5C => { // LD E, H
            cpu.registers.E = cpu.registers.H;
        },
        0x5D => { // LD E, L
            cpu.registers.E = cpu.registers.L;
        },
        0x5E => { // LD E, (HL)
            cpu.registers.E = cpu.memory[cpu.get_hl() as usize];
        },
        0x5F => { // LD E, A
            cpu.registers.E = cpu.registers.A;
        },
        0x60 => { // LD H, B
            cpu.registers.H = cpu.registers.B;
        },
        0x61 => { // LD H, C
            cpu.registers.H = cpu.registers.C;
        },
        0x62 => { // LD H, D
            cpu.registers.H = cpu.registers.D;
        },
        0x63 => { // LD H, E
            cpu.registers.H = cpu.registers.E;
        },
        0x64 => { // LD H, H
            cpu.registers.H = cpu.registers.H;
        },
        0x65 => { // LD H, L
            cpu.registers.H = cpu.registers.L;
        },
        0x66 => { // LD H, (HL)
            cpu.registers.H = cpu.memory[cpu.get_hl() as usize];
        },
        0x67 => { // LD H, A
            cpu.registers.H = cpu.registers.A;
        },
        0x68 => { // LD L, B
            cpu.registers.L = cpu.registers.B;
        },
        0x69 => { // LD L, C
            cpu.registers.L = cpu.registers.C;
        },
        0x6A => { // LD L, D
            cpu.registers.L = cpu.registers.D;
        },
        0x6B => { // LD L, E
            cpu.registers.L = cpu.registers.E;
        },
        0x6C => { // LD L, H
            cpu.registers.L = cpu.registers.H;
        },
        0x6D => { // LD L, L
            cpu.registers.L = cpu.registers.L;
        },
        0x6E => { // LD L, (HL)
            cpu.registers.L = cpu.memory[cpu.get_hl() as usize];
        },
        0x6F => { // LD L, A
            cpu.registers.L = cpu.registers.A;
        },
        0x70 => { // LD (HL), B
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.B;
        },
        0x71 => { // LD (HL), C
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.C;
        },
        0x72 => { // LD (HL), D
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.D;
        },
        0x73 => { // LD (HL), E
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.E;
        },
        0x74 => { // LD (HL), H
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.H;
        },
        0x75 => { // LD (HL), L
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.L;
        },
        0x76 => { // HALT
            //cpu.HALT();
        },
        0x77 => { // LD (HL), A
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.A;
        },
        0x78 => { // LD A, B
            cpu.registers.A = cpu.registers.B;
        },
        0x79 => { // LD A, C
            cpu.registers.A = cpu.registers.C;
        },
        0x7A => { // LD A, D
            cpu.registers.A = cpu.registers.D;
        },
        0x7B => { // LD A, E
            cpu.registers.A = cpu.registers.E;
        },
        0x7C => { // LD A, H
            cpu.registers.A = cpu.registers.H;
        },
        0x7D => { // LD A, L
            cpu.registers.A = cpu.registers.L;
        },
        0x7E => { // LD A, (HL)
            cpu.registers.A = cpu.memory[cpu.get_hl() as usize];
        },
        0x7F => { // LD A, A
            cpu.registers.A = cpu.registers.A;
        },
        0x80 => { // ADD A, B
            cpu.registers.A = cpu.ADD(cpu.registers.A as u16, cpu.registers.B as u16) as u8;
        },
        0x81 => { // ADD A, C
            cpu.registers.A = cpu.ADD(cpu.registers.A as u16, cpu.registers.C as u16) as u8;
        },
        0x82 => { // ADD A, D
            cpu.registers.A = cpu.ADD(cpu.registers.A as u16, cpu.registers.D as u16) as u8;
        },
        0x83 => { // ADD A, E
            cpu.registers.A = cpu.ADD(cpu.registers.A as u16, cpu.registers.E as u16) as u8;
        },
        0x84 => { // ADD A, H
            cpu.registers.A = cpu.ADD(cpu.registers.A as u16, cpu.registers.H as u16) as u8;
        },
        0x85 => { // ADD A, L
            cpu.registers.A = cpu.ADD(cpu.registers.A as u16, cpu.registers.L as u16) as u8;
        },
        0x86 => { // ADD A, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.registers.A = cpu.ADD(cpu.registers.A as u16, value as u16) as u8;
        },
        _ => panic!("Unknown opcode: 0x{:X}", opcode),
    }
}