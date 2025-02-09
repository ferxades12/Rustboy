use crate::CPU;

pub fn execute_opcode(cpu: &mut CPU) -> u8 {
    let opcode = cpu.fetch_byte();
    match opcode {
        0x00 => 1,
        0x01 => {
            // LD BC, u16
            let value = cpu.fetch_word();
            cpu.set_bc(value);
            3
        }
        0x02 => {
            // LD (BC), A
            cpu.memory[cpu.get_bc() as usize] = cpu.registers.A;
            2
        }
        0x03 => {
            // INC BC
            let value = cpu.get_bc().wrapping_add(1);
            cpu.set_bc(value);
            2
        }
        0x04 => {
            // INC B
            cpu.registers.B = cpu.INC(cpu.registers.B);
            1
        }
        0x05 => {
            // DEC B
            cpu.registers.B = cpu.DEC(cpu.registers.B);
            1
        }
        0x06 => {
            // LD B, u8
            let value = cpu.fetch_byte();
            cpu.registers.B = value;
            2
        }
        0x07 => {
            // RLCA
            cpu.RLCA();
            1
        }
        0x08 => {
            // LD (u16), SP
            let word = cpu.fetch_word();
            let low = word as u8;
            let high = (word >> 8) as u8;
            cpu.memory[word as usize] = low;
            cpu.memory[(word + 1) as usize] = high;
            5
        }
        0x09 => {
            // ADD HL, BC
            let result = cpu.ADD16(cpu.get_hl(), cpu.get_bc());
            cpu.set_hl(result);
            2
        }
        0x0A => {
            // LD A, (BC)
            cpu.registers.A = cpu.memory[cpu.get_bc() as usize];
            2
        }
        0x0B => {
            // DEC BC
            let value = cpu.get_bc().wrapping_sub(1);
            cpu.set_bc(value);
            2
        }
        0x0C => {
            // INC C
            cpu.registers.C = cpu.INC(cpu.registers.C);
            1
        }
        0x0D => {
            // DEC C
            cpu.registers.C = cpu.DEC(cpu.registers.C);
            1
        }
        0x0E => {
            // LD C, u8
            let value = cpu.fetch_byte();
            cpu.registers.C = value;
            2
        }
        0x0F => {
            // RRCA
            cpu.RRCA();
            1
        }
        0x10 => {
            // STOP
            cpu.stop_flag = true;
            1
        }
        0x11 => {
            // LD DE, u16
            let value = cpu.fetch_word();
            cpu.set_de(value);
            3
        }
        0x12 => {
            // LD (DE), A
            cpu.memory[cpu.get_de() as usize] = cpu.registers.A;
            2
        }
        0x13 => {
            // INC DE
            let value = cpu.get_de().wrapping_add(1);
            cpu.set_de(value);
            2
        }
        0x14 => {
            // INC D
            cpu.registers.D = cpu.INC(cpu.registers.D);
            1
        }
        0x15 => {
            // DEC D
            cpu.registers.D = cpu.DEC(cpu.registers.D);
            1
        }
        0x16 => {
            // LD D, u8
            let value = cpu.fetch_byte();
            cpu.registers.D = value;
            2
        }
        0x17 => {
            // RLA
            cpu.RLA();
            1
        }
        0x18 => {
            // JR i8
            cpu.JR(true);
            3
        }
        0x19 => {
            // ADD HL, DE
            let result = cpu.ADD16(cpu.get_hl(), cpu.get_de());
            cpu.set_hl(result);
            2
        }
        0x1A => {
            // LD A, (DE)
            cpu.registers.A = cpu.memory[cpu.get_de() as usize];
            2
        }
        0x1B => {
            // DEC DE
            let value = cpu.get_de().wrapping_sub(1);
            cpu.set_de(value);
            2
        }
        0x1C => {
            // INC E
            cpu.registers.E = cpu.INC(cpu.registers.E);
            1
        }
        0x1D => {
            // DEC E
            cpu.registers.E = cpu.DEC(cpu.registers.E);
            1
        }
        0x1E => {
            // LD E, u8
            let value = cpu.fetch_byte();
            cpu.registers.E = value;
            2
        }
        0x1F => {
            // RRA
            cpu.RRA();
            1
        }
        0x20 => {
            // JR NZ, i8
            let cond = !cpu.get_ZF();
            cpu.JR(cond);
            if cond {
                3
            } else {
                2
            }
        }
        0x21 => {
            // LD HL, u16
            let value = cpu.fetch_word();
            cpu.set_hl(value);
            3
        }
        0x22 => {
            // LD (HL+), A
            let hl = cpu.get_hl();
            cpu.memory[hl as usize] = cpu.registers.A;
            cpu.set_hl(hl.wrapping_add(1));
            2
        }
        0x23 => {
            // INC HL
            let value = cpu.get_hl().wrapping_add(1);
            cpu.set_hl(value);
            2
        }
        0x24 => {
            // INC H
            cpu.registers.H = cpu.INC(cpu.registers.H);
            1
        }
        0x25 => {
            // DEC H
            cpu.registers.H = cpu.DEC(cpu.registers.H);
            1
        }
        0x26 => {
            // LD H, u8
            let value = cpu.fetch_byte();
            cpu.registers.H = value;
            2
        }
        0x27 => {
            // DAA
            cpu.DAA();
            1
        }
        0x28 => {
            // JR Z, i8
            let cond = cpu.get_ZF();
            cpu.JR(cond);
            if cond {
                3
            } else {
                2
            }
        }
        0x29 => {
            // ADD HL, HL
            let result = cpu.ADD16(cpu.get_hl(), cpu.get_hl());
            cpu.set_hl(result);
            2
        }
        0x2A => {
            // LD A, (HL+)
            let hl = cpu.get_hl();
            cpu.registers.A = cpu.memory[hl as usize];
            cpu.set_hl(hl.wrapping_add(1));
            2
        }
        0x2B => {
            // DEC HL
            let value = cpu.get_hl().wrapping_sub(1);
            cpu.set_hl(value);
            2
        }
        0x2C => {
            // INC L
            cpu.registers.L = cpu.INC(cpu.registers.L);
            1
        }
        0x2D => {
            // DEC L
            cpu.registers.L = cpu.DEC(cpu.registers.L);
            1
        }
        0x2E => {
            // LD L, u8
            let value = cpu.fetch_byte();
            cpu.registers.L = value;
            2
        }
        0x2F => {
            // CPL
            cpu.CPL();
            1
        }
        0x30 => {
            // JR NC, i8
            let cond = !cpu.get_CF();
            cpu.JR(cond);
            if cond {
                3
            } else {
                2
            }
        }
        0x31 => {
            // LD SP, u16
            let value = cpu.fetch_word();
            cpu.registers.SP = value;
            3
        }
        0x32 => {
            // LD (HL-), A
            let hl = cpu.get_hl();
            cpu.memory[hl as usize] = cpu.registers.A;
            cpu.set_hl(hl.wrapping_sub(1));
            2
        }
        0x33 => {
            // INC SP
            let value = cpu.registers.SP.wrapping_add(1);
            cpu.registers.SP = value;
            2
        }
        0x34 => {
            // INC (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.INC(value);
            cpu.memory[cpu.get_hl() as usize] = result;
            3
        }
        0x35 => {
            // DEC (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.DEC(value);
            cpu.memory[cpu.get_hl() as usize] = result;
            3
        }
        0x36 => {
            // LD (HL), u8
            let value = cpu.fetch_byte();
            cpu.memory[cpu.get_hl() as usize] = value;
            3
        }
        0x37 => {
            // SCF
            cpu.SCF();
            1
        }
        0x38 => {
            // JR C, i8
            let cond = cpu.get_CF();
            cpu.JR(cond);

            if cond {
                3
            } else {
                2
            }
        }
        0x39 => {
            // ADD HL, SP
            let zf = cpu.get_ZF();
            let result = cpu.ADD16(cpu.get_hl(), cpu.registers.SP);
            cpu.set_hl(result);
            cpu.set_ZF(zf);
            2
        }
        0x3A => {
            // LD A, (HL-)
            let hl = cpu.get_hl();
            cpu.registers.A = cpu.memory[hl as usize];
            cpu.set_hl(hl.wrapping_sub(1));
            2
        }
        0x3B => {
            // DEC SP
            let value = cpu.registers.SP.wrapping_sub(1);
            cpu.registers.SP = value;
            2
        }
        0x3C => {
            // INC A
            cpu.registers.A = cpu.INC(cpu.registers.A);
            1
        }
        0x3D => {
            // DEC A
            cpu.registers.A = cpu.DEC(cpu.registers.A);
            1
        }
        0x3E => {
            // LD A, u8
            let value = cpu.fetch_byte();
            cpu.registers.A = value;
            2
        }
        0x3F => {
            // CCF
            cpu.CCF();
            1
        }
        0x40 => {
            // LD B, B
            cpu.registers.B = cpu.registers.B;
            1
        }
        0x41 => {
            // LD B, C
            cpu.registers.B = cpu.registers.C;
            1
        }
        0x42 => {
            // LD B, D
            cpu.registers.B = cpu.registers.D;
            1
        }
        0x43 => {
            // LD B, E
            cpu.registers.B = cpu.registers.E;
            1
        }
        0x44 => {
            // LD B, H
            cpu.registers.B = cpu.registers.H;
            1
        }
        0x45 => {
            // LD B, L
            cpu.registers.B = cpu.registers.L;
            1
        }
        0x46 => {
            // LD B, (HL)
            cpu.registers.B = cpu.memory[cpu.get_hl() as usize];
            2
        }
        0x47 => {
            // LD B, A
            cpu.registers.B = cpu.registers.A;
            1
        }
        0x48 => {
            // LD C, B
            cpu.registers.C = cpu.registers.B;
            1
        }
        0x49 => {
            // LD C, C
            cpu.registers.C = cpu.registers.C;
            1
        }
        0x4A => {
            // LD C, D
            cpu.registers.C = cpu.registers.D;
            1
        }
        0x4B => {
            // LD C, E
            cpu.registers.C = cpu.registers.E;
            1
        }
        0x4C => {
            // LD C, H
            cpu.registers.C = cpu.registers.H;
            1
        }
        0x4D => {
            // LD C, L
            cpu.registers.C = cpu.registers.L;
            1
        }
        0x4E => {
            // LD C, (HL)
            cpu.registers.C = cpu.memory[cpu.get_hl() as usize];
            2
        }
        0x4F => {
            // LD C, A
            cpu.registers.C = cpu.registers.A;
            1
        }
        0x50 => {
            // LD D, B
            cpu.registers.D = cpu.registers.B;
            1
        }
        0x51 => {
            // LD D, C
            cpu.registers.D = cpu.registers.C;
            1
        }
        0x52 => {
            // LD D, D
            cpu.registers.D = cpu.registers.D;
            1
        }
        0x53 => {
            // LD D, E
            cpu.registers.D = cpu.registers.E;
            1
        }
        0x54 => {
            // LD D, H
            cpu.registers.D = cpu.registers.H;
            1
        }
        0x55 => {
            // LD D, L
            cpu.registers.D = cpu.registers.L;
            1
        }
        0x56 => {
            // LD D, (HL)
            cpu.registers.D = cpu.memory[cpu.get_hl() as usize];
            2
        }
        0x57 => {
            // LD D, A
            cpu.registers.D = cpu.registers.A;
            1
        }
        0x58 => {
            // LD E, B
            cpu.registers.E = cpu.registers.B;
            1
        }
        0x59 => {
            // LD E, C
            cpu.registers.E = cpu.registers.C;
            1
        }
        0x5A => {
            // LD E, D
            cpu.registers.E = cpu.registers.D;
            1
        }
        0x5B => {
            // LD E, E
            cpu.registers.E = cpu.registers.E;
            1
        }
        0x5C => {
            // LD E, H
            cpu.registers.E = cpu.registers.H;
            1
        }
        0x5D => {
            // LD E, L
            cpu.registers.E = cpu.registers.L;
            1
        }
        0x5E => {
            // LD E, (HL)
            cpu.registers.E = cpu.memory[cpu.get_hl() as usize];
            2
        }
        0x5F => {
            // LD E, A
            cpu.registers.E = cpu.registers.A;
            1
        }
        0x60 => {
            // LD H, B
            cpu.registers.H = cpu.registers.B;
            1
        }
        0x61 => {
            // LD H, C
            cpu.registers.H = cpu.registers.C;
            1
        }
        0x62 => {
            // LD H, D
            cpu.registers.H = cpu.registers.D;
            1
        }
        0x63 => {
            // LD H, E
            cpu.registers.H = cpu.registers.E;
            1
        }
        0x64 => {
            // LD H, H
            cpu.registers.H = cpu.registers.H;
            1
        }
        0x65 => {
            // LD H, L
            cpu.registers.H = cpu.registers.L;
            1
        }
        0x66 => {
            // LD H, (HL)
            cpu.registers.H = cpu.memory[cpu.get_hl() as usize];
            2
        }
        0x67 => {
            // LD H, A
            cpu.registers.H = cpu.registers.A;
            1
        }
        0x68 => {
            // LD L, B
            cpu.registers.L = cpu.registers.B;
            1
        }
        0x69 => {
            // LD L, C
            cpu.registers.L = cpu.registers.C;
            1
        }
        0x6A => {
            // LD L, D
            cpu.registers.L = cpu.registers.D;
            1
        }
        0x6B => {
            // LD L, E
            cpu.registers.L = cpu.registers.E;
            1
        }
        0x6C => {
            // LD L, H
            cpu.registers.L = cpu.registers.H;
            1
        }
        0x6D => {
            // LD L, L
            cpu.registers.L = cpu.registers.L;
            1
        }
        0x6E => {
            // LD L, (HL)
            cpu.registers.L = cpu.memory[cpu.get_hl() as usize];
            2
        }
        0x6F => {
            // LD L, A
            cpu.registers.L = cpu.registers.A;
            1
        }
        0x70 => {
            // LD (HL), B
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.B;
            2
        }
        0x71 => {
            // LD (HL), C
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.C;
            2
        }
        0x72 => {
            // LD (HL), D
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.D;
            2
        }
        0x73 => {
            // LD (HL), E
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.E;
            2
        }
        0x74 => {
            // LD (HL), H
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.H;
            2
        }
        0x75 => {
            // LD (HL), L
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.L;
            2
        }
        0x76 => {
            // HALT
            cpu.halt_flag = true;
            1
        }
        0x77 => {
            // LD (HL), A
            cpu.memory[cpu.get_hl() as usize] = cpu.registers.A;
            2
        }
        0x78 => {
            // LD A, B
            cpu.registers.A = cpu.registers.B;
            1
        }
        0x79 => {
            // LD A, C
            cpu.registers.A = cpu.registers.C;
            1
        }
        0x7A => {
            // LD A, D
            cpu.registers.A = cpu.registers.D;
            1
        }
        0x7B => {
            // LD A, E
            cpu.registers.A = cpu.registers.E;
            1
        }
        0x7C => {
            // LD A, H
            cpu.registers.A = cpu.registers.H;
            1
        }
        0x7D => {
            // LD A, L
            cpu.registers.A = cpu.registers.L;
            1
        }
        0x7E => {
            // LD A, (HL)
            cpu.registers.A = cpu.memory[cpu.get_hl() as usize];
            2
        }
        0x7F => {
            // LD A, A
            cpu.registers.A = cpu.registers.A;
            1
        }
        0x80 => {
            // ADD A, B
            cpu.registers.A = cpu.ADD8(cpu.registers.B);
            1
        }
        0x81 => {
            // ADD A, C
            cpu.registers.A = cpu.ADD8(cpu.registers.C);
            1
        }
        0x82 => {
            // ADD A, D
            cpu.registers.A = cpu.ADD8(cpu.registers.D);
            1
        }
        0x83 => {
            // ADD A, E
            cpu.registers.A = cpu.ADD8(cpu.registers.E);
            1
        }
        0x84 => {
            // ADD A, H
            cpu.registers.A = cpu.ADD8(cpu.registers.H);
            1
        }
        0x85 => {
            // ADD A, L
            cpu.registers.A = cpu.ADD8(cpu.registers.L);
            1
        }
        0x86 => {
            // ADD A, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.registers.A = cpu.ADD8(value);
            2
        }
        0x87 => {
            // ADD A, A
            cpu.registers.A = cpu.ADD8(cpu.registers.A);
            1
        }
        0x88 => {
            // ADC A, B
            cpu.registers.A = cpu.ADC(cpu.registers.B);
            2
        }
        0x89 => {
            // ADC A, C
            cpu.registers.A = cpu.ADC(cpu.registers.C);
            1
        }
        0x8A => {
            // ADC A, D
            cpu.registers.A = cpu.ADC(cpu.registers.D);
            1
        }
        0x8B => {
            // ADC A, E
            cpu.registers.A = cpu.ADC(cpu.registers.E);
            1
        }
        0x8C => {
            // ADC A, H
            cpu.registers.A = cpu.ADC(cpu.registers.H);
            1
        }
        0x8D => {
            // ADC A, L
            cpu.registers.A = cpu.ADC(cpu.registers.L);
            1
        }
        0x8E => {
            // ADC A, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.registers.A = cpu.ADC(value);
            2
        }
        0x8F => {
            // ADC A, A
            cpu.registers.A = cpu.ADC(cpu.registers.A);
            1
        }
        0x90 => {
            // SUB A, B
            cpu.registers.A = cpu.SUB(cpu.registers.B);
            1
        }
        0x91 => {
            // SUB A, C
            cpu.registers.A = cpu.SUB(cpu.registers.C);
            1
        }
        0x92 => {
            // SUB A, D
            cpu.registers.A = cpu.SUB(cpu.registers.D);
            1
        }
        0x93 => {
            // SUB A, E
            cpu.registers.A = cpu.SUB(cpu.registers.E);
            1
        }
        0x94 => {
            // SUB A, H
            cpu.registers.A = cpu.SUB(cpu.registers.H);
            1
        }
        0x95 => {
            // SUB A, L
            cpu.registers.A = cpu.SUB(cpu.registers.L);
            1
        }
        0x96 => {
            // SUB A, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.registers.A = cpu.SUB(value);
            2
        }
        0x97 => {
            // SUB A, A
            cpu.registers.A = cpu.SUB(cpu.registers.A);
            1
        }
        0x98 => {
            // SBC A, B
            cpu.registers.A = cpu.SBC(cpu.registers.B);
            1
        }
        0x99 => {
            // SBC A, C
            cpu.registers.A = cpu.SBC(cpu.registers.C);
            1
        }
        0x9A => {
            // SBC A, D
            cpu.registers.A = cpu.SBC(cpu.registers.D);
            1
        }
        0x9B => {
            // SBC A, E
            cpu.registers.A = cpu.SBC(cpu.registers.E);
            1
        }
        0x9C => {
            // SBC A, H
            cpu.registers.A = cpu.SBC(cpu.registers.H);
            1
        }
        0x9D => {
            // SBC A, L
            cpu.registers.A = cpu.SBC(cpu.registers.L);
            1
        }
        0x9E => {
            // SBC A, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.registers.A = cpu.SBC(value);
            2
        }
        0x9F => {
            // SBC A, A
            cpu.registers.A = cpu.SBC(cpu.registers.A);
            1
        }
        0xA0 => {
            // AND A, B
            cpu.AND(cpu.registers.B);
            1
        }
        0xA1 => {
            // AND A, C
            cpu.AND(cpu.registers.C);
            1
        }
        0xA2 => {
            // AND A, D
            cpu.AND(cpu.registers.D);
            1
        }
        0xA3 => {
            // AND A, E
            cpu.AND(cpu.registers.E);
            1
        }
        0xA4 => {
            // AND A, H
            cpu.AND(cpu.registers.H);
            1
        }
        0xA5 => {
            // AND A, L
            cpu.AND(cpu.registers.L);
            1
        }
        0xA6 => {
            // AND A, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.AND(value);
            2
        }
        0xA7 => {
            // AND A, A
            cpu.AND(cpu.registers.A);
            1
        }
        0xA8 => {
            // XOR A, B
            cpu.XOR(cpu.registers.B);
            1
        }
        0xA9 => {
            // XOR A, C
            cpu.XOR(cpu.registers.C);
            1
        }
        0xAA => {
            // XOR A, D
            cpu.XOR(cpu.registers.D);
            1
        }
        0xAB => {
            // XOR A, E
            cpu.XOR(cpu.registers.E);
            1
        }
        0xAC => {
            // XOR A, H
            cpu.XOR(cpu.registers.H);
            1
        }
        0xAD => {
            // XOR A, L
            cpu.XOR(cpu.registers.L);
            1
        }
        0xAE => {
            // XOR A, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.XOR(value);
            2
        }
        0xAF => {
            // XOR A, A
            cpu.XOR(cpu.registers.A);
            1
        }
        0xB0 => {
            // OR A, B
            cpu.OR(cpu.registers.B);
            1
        }
        0xB1 => {
            // OR A, C
            cpu.OR(cpu.registers.C);
            1
        }
        0xB2 => {
            // OR A, D
            cpu.OR(cpu.registers.D);
            1
        }
        0xB3 => {
            // OR A, E
            cpu.OR(cpu.registers.E);
            1
        }
        0xB4 => {
            // OR A, H
            cpu.OR(cpu.registers.H);
            1
        }
        0xB5 => {
            // OR A, L
            cpu.OR(cpu.registers.L);
            1
        }
        0xB6 => {
            // OR A, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.OR(value);
            2
        }
        0xB7 => {
            // OR A, A
            cpu.OR(cpu.registers.A);
            1
        }
        0xB8 => {
            // CP A, B
            cpu.CP(cpu.registers.B);
            1
        }
        0xB9 => {
            // CP A, C
            cpu.CP(cpu.registers.C);
            1
        }
        0xBA => {
            // CP A, D
            cpu.CP(cpu.registers.D);
            1
        }
        0xBB => {
            // CP A, E
            cpu.CP(cpu.registers.E);
            1
        }
        0xBC => {
            // CP A, H
            cpu.CP(cpu.registers.H);
            1
        }
        0xBD => {
            // CP A, L
            cpu.CP(cpu.registers.L);
            1
        }
        0xBE => {
            // CP A, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.CP(value);
            2
        }
        0xBF => {
            // CP A, A
            cpu.CP(cpu.registers.A);
            1
        }
        0xC0 => {
            // RET NZ
            let cond = !cpu.get_ZF();
            cpu.RET(cond);

            if cond {
                5
            } else {
                2
            }
        }
        0xC1 => {
            // POP BC
            let value = cpu.POP();
            cpu.set_bc(value);
            3
        }
        0xC2 => {
            // JP NZ, u16
            let cond = !cpu.get_ZF();
            cpu.JP(cond);
            if cond {
                4
            } else {
                3
            }
        }
        0xC3 => {
            // JP u16
            cpu.JP(true);
            4
        }
        0xC4 => {
            // CALL NZ, u16
            let cond = !cpu.get_ZF();
            cpu.CALL(cond);
            if cond {
                6
            } else {
                3
            }
        }
        0xC5 => {
            // PUSH BC
            cpu.PUSH(cpu.get_bc());
            4
        }
        0xC6 => {
            // ADD A, u8
            let value = cpu.fetch_byte();
            cpu.registers.A = cpu.ADD8(value);
            2
        }
        0xC7 => {
            // RST 00H
            cpu.RST(0x00);
            4
        }
        0xC8 => {
            // RET Z
            let cond = cpu.get_ZF();
            cpu.RET(cond);
            if cond {
                5
            } else {
                2
            }
        }
        0xC9 => {
            // RET
            cpu.RET(true);
            4
        }
        0xCA => {
            // JP Z, u16
            let cond = cpu.get_ZF();
            cpu.JP(cond);
            if cond {
                4
            } else {
                3
            }
        }
        0xCB => {
            // PREFIX CB
            execute_cb_opcode(cpu)
        }
        0xCC => {
            // CALL Z, u16
            let cond = cpu.get_ZF();
            cpu.CALL(cond);
            if cond {
                6
            } else {
                3
            }
        }
        0xCD => {
            // CALL u16
            cpu.CALL(true);
            6
        }
        0xCE => {
            // ADC A, u8
            let value = cpu.fetch_byte();
            cpu.registers.A = cpu.ADC(value);
            2
        }
        0xCF => {
            // RST 08H
            cpu.RST(0x08);
            4
        }
        0xD0 => {
            // RET NC
            let cond = !cpu.get_CF();
            cpu.RET(cond);
            if cond {
                5
            } else {
                2
            }
        }
        0xD1 => {
            // POP DE
            let value = cpu.POP();
            cpu.set_de(value);
            3
        }
        0xD2 => {
            // JP NC, u16
            let cond = !cpu.get_CF();
            cpu.JP(cond);
            if cond {
                4
            } else {
                3
            }
        }
        0xD4 => {
            // CALL NC, u16
            let cond = !cpu.get_CF();
            cpu.CALL(cond);
            if cond {
                6
            } else {
                3
            }
        }
        0xD5 => {
            // PUSH DE
            cpu.PUSH(cpu.get_de());
            4
        }
        0xD6 => {
            // SUB A, u8
            let value = cpu.fetch_byte();
            cpu.registers.A = cpu.SUB(value);
            2
        }
        0xD7 => {
            // RST 10H
            cpu.RST(0x10);
            4
        }
        0xD8 => {
            // RET C
            cpu.RET(cpu.get_CF());
            5
        }
        0xD9 => {
            // RETI
            cpu.RET(true);
            cpu.ei_flag = true;
            4
        }
        0xDA => {
            // JP C, u16
            let cond = cpu.get_CF();
            cpu.JP(cond);
            if cond {
                4
            } else {
                3
            }
        }
        0xDC => {
            // CALL C, u16
            let cond = cpu.get_CF();
            cpu.CALL(cond);
            if cond {
                6
            } else {
                3
            }
        }
        0xDE => {
            // SBC A, u8
            let value = cpu.fetch_byte();
            cpu.registers.A = cpu.SBC(value);
            2
        }
        0xDF => {
            // RST 18H
            cpu.RST(0x18);
            4
        }
        0xE0 => {
            // LD (FF00 + u8), A
            let value = cpu.fetch_byte();
            cpu.memory[0xFF00 + value as usize] = cpu.registers.A;
            3
        }
        0xE1 => {
            // POP HL
            let value = cpu.POP();
            cpu.set_hl(value);
            3
        }
        0xE2 => {
            // LD (FF00 + C), A
            cpu.memory[0xFF00 + cpu.registers.C as usize] = cpu.registers.A;
            2
        }
        0xE5 => {
            // PUSH HL
            cpu.PUSH(cpu.get_hl());
            4
        }
        0xE6 => {
            // AND A, u8
            let value = cpu.fetch_byte();
            cpu.AND(value);
            2
        }
        0xE7 => {
            // RST 20H
            cpu.RST(0x20);
            4
        }
        0xE8 => {
            // ADD SP, i8
            let value: i8 = cpu.fetch_byte() as i8;
            let (result, carry) = cpu.registers.SP.overflowing_add(value as u16);
            let half_carry = (cpu.registers.SP & 0xF) + ((value as u16) & 0xF) > 0xF;
            cpu.update_flags(false, carry, half_carry, false);
            cpu.registers.SP = result;
            4
        }
        0xE9 => {
            // JP HL
            cpu.JP(false);
            1
        }
        0xEA => {
            // LD (u16), A
            let value = cpu.fetch_word();
            cpu.memory[value as usize] = cpu.registers.A;
            4
        }
        0xEE => {
            // XOR A, u8
            let value = cpu.fetch_byte();
            cpu.XOR(value);
            2
        }
        0xEF => {
            // RST 28H
            cpu.RST(0x28);
            4
        }
        0xF0 => {
            // LD A, (FF00 + u8)
            let value = cpu.fetch_byte();
            cpu.registers.A = cpu.memory[0xFF00 + value as usize];
            3
        }
        0xF1 => {
            // POP AF
            let value = cpu.POP();
            cpu.set_af(value);
            3
        }
        0xF2 => {
            // LD A, (FF00 + C)
            cpu.registers.A = cpu.memory[0xFF00 + cpu.registers.C as usize];
            2
        }
        0xF3 => {
            // Disable Interrupt
            cpu.ei_flag = false;
            cpu.registers.IME = false;
            1
        }
        0xF5 => {
            // PUSH AF
            cpu.PUSH(cpu.get_af());
            4
        }
        0xF6 => {
            // OR A, u8
            let value = cpu.fetch_byte();
            cpu.OR(value);
            2
        }
        0xF7 => {
            // RST 30H
            cpu.RST(0x30);
            4
        }
        0xF8 => {
            // LD HL, SP + i8
            let value: i8 = cpu.fetch_byte() as i8;
            let (result, carry) = cpu.registers.SP.overflowing_add(value as u16);
            let half_carry = (cpu.registers.SP & 0xF) + ((value as u16) & 0xF) > 0xF;
            cpu.update_flags(false, carry, half_carry, false);
            cpu.set_hl(result);
            3
        }
        0xF9 => {
            // LD SP, HL
            cpu.registers.SP = cpu.get_hl();
            2
        }
        0xFA => {
            // LD A, (u16)
            let value = cpu.fetch_word();
            cpu.registers.A = cpu.memory[value as usize];
            4
        }
        0xFB => {
            // Enable Interrupt
            cpu.ei_flag = true;
            1
        }
        0xFE => {
            // CP A, u8
            let value = cpu.fetch_byte();
            cpu.CP(value);
            2
        }
        0xFF => {
            // RST 38H
            cpu.RST(0x38);
            4
        }
        _ => panic!("Unknown opcode: 0x{:X}", opcode),
    }
}

fn execute_cb_opcode(cpu: &mut CPU) -> u8 {
    let op_code = cpu.fetch_byte();
    match op_code {
        0x00 => {
            // RLC B
            cpu.registers.B = cpu.RLC(cpu.registers.B);
            2
        }
        0x01 => {
            // RLC C
            cpu.registers.C = cpu.RLC(cpu.registers.C);
            2
        }
        0x02 => {
            // RLC D
            cpu.registers.D = cpu.RLC(cpu.registers.D);
            2
        }
        0x03 => {
            // RLC E
            cpu.registers.E = cpu.RLC(cpu.registers.E);
            2
        }
        0x04 => {
            // RLC H
            cpu.registers.H = cpu.RLC(cpu.registers.H);
            2
        }
        0x05 => {
            // RLC L
            cpu.registers.L = cpu.RLC(cpu.registers.L);
            2
        }
        0x06 => {
            // RLC (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.RLC(value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0x07 => {
            // RLC A
            cpu.registers.A = cpu.RLC(cpu.registers.A);
            2
        }
        0x08 => {
            // RRC B
            cpu.registers.B = cpu.RRC(cpu.registers.B);
            2
        }
        0x09 => {
            // RRC C
            cpu.registers.C = cpu.RRC(cpu.registers.C);
            2
        }
        0x0A => {
            // RRC D
            cpu.registers.D = cpu.RRC(cpu.registers.D);
            2
        }
        0x0B => {
            // RRC E
            cpu.registers.E = cpu.RRC(cpu.registers.E);
            2
        }
        0x0C => {
            // RRC H
            cpu.registers.H = cpu.RRC(cpu.registers.H);
            2
        }
        0x0D => {
            // RRC L
            cpu.registers.L = cpu.RRC(cpu.registers.L);
            2
        }
        0x0E => {
            // RRC (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.RRC(value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0x0F => {
            // RRC A
            cpu.registers.A = cpu.RRC(cpu.registers.A);
            2
        }
        0x10 => {
            // RL B
            cpu.registers.B = cpu.RL(cpu.registers.B);
            2
        }
        0x11 => {
            // RL C
            cpu.registers.C = cpu.RL(cpu.registers.C);
            2
        }
        0x12 => {
            // RL D
            cpu.registers.D = cpu.RL(cpu.registers.D);
            2
        }
        0x13 => {
            // RL E
            cpu.registers.E = cpu.RL(cpu.registers.E);
            2
        }
        0x14 => {
            // RL H
            cpu.registers.H = cpu.RL(cpu.registers.H);
            2
        }
        0x15 => {
            // RL L
            cpu.registers.L = cpu.RL(cpu.registers.L);
            2
        }
        0x16 => {
            // RL (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.RL(value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0x17 => {
            // RL A
            cpu.registers.A = cpu.RL(cpu.registers.A);
            2
        }
        0x18 => {
            // RR B
            cpu.registers.B = cpu.RR(cpu.registers.B);
            2
        }
        0x19 => {
            // RR C
            cpu.registers.C = cpu.RR(cpu.registers.C);
            2
        }
        0x1A => {
            // RR D
            cpu.registers.D = cpu.RR(cpu.registers.D);
            2
        }
        0x1B => {
            // RR E
            cpu.registers.E = cpu.RR(cpu.registers.E);
            2
        }
        0x1C => {
            // RR H
            cpu.registers.H = cpu.RR(cpu.registers.H);
            2
        }
        0x1D => {
            // RR L
            cpu.registers.L = cpu.RR(cpu.registers.L);
            2
        }
        0x1E => {
            // RR (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.RR(value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0x1F => {
            // RR A
            cpu.registers.A = cpu.RR(cpu.registers.A);
            2
        }
        0x20 => {
            // SLA B
            cpu.registers.B = cpu.SLA(cpu.registers.B);
            2
        }
        0x21 => {
            // SLA C
            cpu.registers.C = cpu.SLA(cpu.registers.C);
            2
        }
        0x22 => {
            // SLA D
            cpu.registers.D = cpu.SLA(cpu.registers.D);
            2
        }
        0x23 => {
            // SLA E
            cpu.registers.E = cpu.SLA(cpu.registers.E);
            2
        }
        0x24 => {
            // SLA H
            cpu.registers.H = cpu.SLA(cpu.registers.H);
            2
        }
        0x25 => {
            // SLA L
            cpu.registers.L = cpu.SLA(cpu.registers.L);
            2
        }
        0x26 => {
            // SLA (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.SLA(value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0x27 => {
            // SLA A
            cpu.registers.A = cpu.SLA(cpu.registers.A);
            2
        }
        0x28 => {
            // SRA B
            cpu.registers.B = cpu.SRA(cpu.registers.B);
            2
        }
        0x29 => {
            // SRA C
            cpu.registers.C = cpu.SRA(cpu.registers.C);
            2
        }
        0x2A => {
            // SRA D
            cpu.registers.D = cpu.SRA(cpu.registers.D);
            2
        }
        0x2B => {
            // SRA E
            cpu.registers.E = cpu.SRA(cpu.registers.E);
            2
        }
        0x2C => {
            // SRA H
            cpu.registers.H = cpu.SRA(cpu.registers.H);
            2
        }
        0x2D => {
            // SRA L
            cpu.registers.L = cpu.SRA(cpu.registers.L);
            2
        }
        0x2E => {
            // SRA (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.SRA(value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0x2F => {
            // SRA A
            cpu.registers.A = cpu.SRA(cpu.registers.A);
            2
        }
        0x30 => {
            // SWAP B
            cpu.registers.B = cpu.SWAP(cpu.registers.B);
            2
        }
        0x31 => {
            // SWAP C
            cpu.registers.C = cpu.SWAP(cpu.registers.C);
            2
        }
        0x32 => {
            // SWAP D
            cpu.registers.D = cpu.SWAP(cpu.registers.D);
            2
        }
        0x33 => {
            // SWAP E
            cpu.registers.E = cpu.SWAP(cpu.registers.E);
            2
        }
        0x34 => {
            // SWAP H
            cpu.registers.H = cpu.SWAP(cpu.registers.H);
            2
        }
        0x35 => {
            // SWAP L
            cpu.registers.L = cpu.SWAP(cpu.registers.L);
            2
        }
        0x36 => {
            // SWAP (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.SWAP(value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0x37 => {
            // SWAP A
            cpu.registers.A = cpu.SWAP(cpu.registers.A);
            2
        }
        0x38 => {
            // SRL B
            cpu.registers.B = cpu.SRL(cpu.registers.B);
            2
        }
        0x39 => {
            // SRL C
            cpu.registers.C = cpu.SRL(cpu.registers.C);
            2
        }
        0x3A => {
            // SRL D
            cpu.registers.D = cpu.SRL(cpu.registers.D);
            2
        }
        0x3B => {
            // SRL E
            cpu.registers.E = cpu.SRL(cpu.registers.E);
            2
        }
        0x3C => {
            // SRL H
            cpu.registers.H = cpu.SRL(cpu.registers.H);
            2
        }
        0x3D => {
            // SRL L
            cpu.registers.L = cpu.SRL(cpu.registers.L);
            2
        }
        0x3E => {
            // SRL (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.SRL(value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0x3F => {
            // SRL A
            cpu.registers.A = cpu.SRL(cpu.registers.A);
            2
        }
        0x40 => {
            // BIT 0, B
            cpu.BIT(0, cpu.registers.B);
            2
        }
        0x41 => {
            // BIT 0, C
            cpu.BIT(0, cpu.registers.C);
            2
        }
        0x42 => {
            // BIT 0, D
            cpu.BIT(0, cpu.registers.D);
            2
        }
        0x43 => {
            // BIT 0, E
            cpu.BIT(0, cpu.registers.E);
            2
        }
        0x44 => {
            // BIT 0, H
            cpu.BIT(0, cpu.registers.H);
            2
        }
        0x45 => {
            // BIT 0, L
            cpu.BIT(0, cpu.registers.L);
            2
        }
        0x46 => {
            // BIT 0, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.BIT(0, value);
            4
        }
        0x47 => {
            // BIT 0, A
            cpu.BIT(0, cpu.registers.A);
            2
        }
        0x48 => {
            // BIT 1, B
            cpu.BIT(1, cpu.registers.B);
            2
        }
        0x49 => {
            // BIT 1, C
            cpu.BIT(1, cpu.registers.C);
            2
        }
        0x4A => {
            // BIT 1, D
            cpu.BIT(1, cpu.registers.D);
            2
        }
        0x4B => {
            // BIT 1, E
            cpu.BIT(1, cpu.registers.E);
            2
        }
        0x4C => {
            // BIT 1, H
            cpu.BIT(1, cpu.registers.H);
            2
        }
        0x4D => {
            // BIT 1, L
            cpu.BIT(1, cpu.registers.L);
            2
        }
        0x4E => {
            // BIT 1, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.BIT(1, value);
            4
        }
        0x4F => {
            // BIT 1, A
            cpu.BIT(1, cpu.registers.A);
            2
        }
        0x50 => {
            // BIT 2, B
            cpu.BIT(2, cpu.registers.B);
            2
        }
        0x51 => {
            // BIT 2, C
            cpu.BIT(2, cpu.registers.C);
            2
        }
        0x52 => {
            // BIT 2, D
            cpu.BIT(2, cpu.registers.D);
            2
        }
        0x53 => {
            // BIT 2, E
            cpu.BIT(2, cpu.registers.E);
            2
        }
        0x54 => {
            // BIT 2, H
            cpu.BIT(2, cpu.registers.H);
            2
        }
        0x55 => {
            // BIT 2, L
            cpu.BIT(2, cpu.registers.L);
            2
        }
        0x56 => {
            // BIT 2, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.BIT(2, value);
            4
        }
        0x57 => {
            // BIT 2, A
            cpu.BIT(2, cpu.registers.A);
            2
        }
        0x58 => {
            // BIT 3, B
            cpu.BIT(3, cpu.registers.B);
            2
        }
        0x59 => {
            // BIT 3, C
            cpu.BIT(3, cpu.registers.C);
            2
        }
        0x5A => {
            // BIT 3, D
            cpu.BIT(3, cpu.registers.D);
            2
        }
        0x5B => {
            // BIT 3, E
            cpu.BIT(3, cpu.registers.E);
            2
        }
        0x5C => {
            // BIT 3, H
            cpu.BIT(3, cpu.registers.H);
            2
        }
        0x5D => {
            // BIT 3, L
            cpu.BIT(3, cpu.registers.L);
            2
        }
        0x5E => {
            // BIT 3, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.BIT(3, value);
            4
        }
        0x5F => {
            // BIT 3, A
            cpu.BIT(3, cpu.registers.A);
            2
        }
        0x60 => {
            // BIT 4, B
            cpu.BIT(4, cpu.registers.B);
            2
        }
        0x61 => {
            // BIT 4, C
            cpu.BIT(4, cpu.registers.C);
            2
        }
        0x62 => {
            // BIT 4, D
            cpu.BIT(4, cpu.registers.D);
            2
        }
        0x63 => {
            // BIT 4, E
            cpu.BIT(4, cpu.registers.E);
            2
        }
        0x64 => {
            // BIT 4, H
            cpu.BIT(4, cpu.registers.H);
            2
        }
        0x65 => {
            // BIT 4, L
            cpu.BIT(4, cpu.registers.L);
            2
        }
        0x66 => {
            // BIT 4, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.BIT(4, value);
            4
        }
        0x67 => {
            // BIT 4, A
            cpu.BIT(4, cpu.registers.A);
            2
        }
        0x68 => {
            // BIT 5, B
            cpu.BIT(5, cpu.registers.B);
            2
        }
        0x69 => {
            // BIT 5, C
            cpu.BIT(5, cpu.registers.C);
            2
        }
        0x6A => {
            // BIT 5, D
            cpu.BIT(5, cpu.registers.D);
            2
        }
        0x6B => {
            // BIT 5, E
            cpu.BIT(5, cpu.registers.E);
            2
        }
        0x6C => {
            // BIT 5, H
            cpu.BIT(5, cpu.registers.H);
            2
        }
        0x6D => {
            // BIT 5, L
            cpu.BIT(5, cpu.registers.L);
            2
        }
        0x6E => {
            // BIT 5, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.BIT(5, value);
            4
        }
        0x6F => {
            // BIT 5, A
            cpu.BIT(5, cpu.registers.A);
            2
        }
        0x70 => {
            // BIT 6, B
            cpu.BIT(6, cpu.registers.B);
            2
        }
        0x71 => {
            // BIT 6, C
            cpu.BIT(6, cpu.registers.C);
            2
        }
        0x72 => {
            // BIT 6, D
            cpu.BIT(6, cpu.registers.D);
            2
        }
        0x73 => {
            // BIT 6, E
            cpu.BIT(6, cpu.registers.E);
            2
        }
        0x74 => {
            // BIT 6, H
            cpu.BIT(6, cpu.registers.H);
            2
        }
        0x75 => {
            // BIT 6, L
            cpu.BIT(6, cpu.registers.L);
            2
        }
        0x76 => {
            // BIT 6, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.BIT(6, value);
            4
        }
        0x77 => {
            // BIT 6, A
            cpu.BIT(6, cpu.registers.A);
            2
        }
        0x78 => {
            // BIT 7, B
            cpu.BIT(7, cpu.registers.B);
            2
        }
        0x79 => {
            // BIT 7, C
            cpu.BIT(7, cpu.registers.C);
            2
        }
        0x7A => {
            // BIT 7, D
            cpu.BIT(7, cpu.registers.D);
            2
        }
        0x7B => {
            // BIT 7, E
            cpu.BIT(7, cpu.registers.E);
            2
        }
        0x7C => {
            // BIT 7, H
            cpu.BIT(7, cpu.registers.H);
            2
        }
        0x7D => {
            // BIT 7, L
            cpu.BIT(7, cpu.registers.L);
            2
        }
        0x7E => {
            // BIT 7, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            cpu.BIT(7, value);
            4
        }
        0x7F => {
            // BIT 7, A
            cpu.BIT(7, cpu.registers.A);
            2
        }
        0x80 => {
            // RES 0, B
            cpu.registers.B = cpu.RES(0, cpu.registers.B);
            2
        }
        0x81 => {
            // RES 0, C
            cpu.registers.C = cpu.RES(0, cpu.registers.C);
            2
        }
        0x82 => {
            // RES 0, D
            cpu.registers.D = cpu.RES(0, cpu.registers.D);
            2
        }
        0x83 => {
            // RES 0, E
            cpu.registers.E = cpu.RES(0, cpu.registers.E);
            2
        }
        0x84 => {
            // RES 0, H
            cpu.registers.H = cpu.RES(0, cpu.registers.H);
            2
        }
        0x85 => {
            // RES 0, L
            cpu.registers.L = cpu.RES(0, cpu.registers.L);
            2
        }
        0x86 => {
            // RES 0, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.RES(0, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0x87 => {
            // RES 0, A
            cpu.registers.A = cpu.RES(0, cpu.registers.A);
            2
        }
        0x88 => {
            // RES 1, B
            cpu.registers.B = cpu.RES(1, cpu.registers.B);
            2
        }
        0x89 => {
            // RES 1, C
            cpu.registers.C = cpu.RES(1, cpu.registers.C);
            2
        }
        0x8A => {
            // RES 1, D
            cpu.registers.D = cpu.RES(1, cpu.registers.D);
            2
        }
        0x8B => {
            // RES 1, E
            cpu.registers.E = cpu.RES(1, cpu.registers.E);
            2
        }
        0x8C => {
            // RES 1, H
            cpu.registers.H = cpu.RES(1, cpu.registers.H);
            2
        }
        0x8D => {
            // RES 1, L
            cpu.registers.L = cpu.RES(1, cpu.registers.L);
            2
        }
        0x8E => {
            // RES 1, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.RES(1, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0x8F => {
            // RES 1, A
            cpu.registers.A = cpu.RES(1, cpu.registers.A);
            2
        }
        0x90 => {
            // RES 2, B
            cpu.registers.B = cpu.RES(2, cpu.registers.B);
            2
        }
        0x91 => {
            // RES 2, C
            cpu.registers.C = cpu.RES(2, cpu.registers.C);
            2
        }
        0x92 => {
            // RES 2, D
            cpu.registers.D = cpu.RES(2, cpu.registers.D);
            2
        }
        0x93 => {
            // RES 2, E
            cpu.registers.E = cpu.RES(2, cpu.registers.E);
            2
        }
        0x94 => {
            // RES 2, H
            cpu.registers.H = cpu.RES(2, cpu.registers.H);
            2
        }
        0x95 => {
            // RES 2, L
            cpu.registers.L = cpu.RES(2, cpu.registers.L);
            2
        }
        0x96 => {
            // RES 2, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.RES(2, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0x97 => {
            // RES 2, A
            cpu.registers.A = cpu.RES(2, cpu.registers.A);
            2
        }
        0x98 => {
            // RES 3, B
            cpu.registers.B = cpu.RES(3, cpu.registers.B);
            2
        }
        0x99 => {
            // RES 3, C
            cpu.registers.C = cpu.RES(3, cpu.registers.C);
            2
        }
        0x9A => {
            // RES 3, D
            cpu.registers.D = cpu.RES(3, cpu.registers.D);
            2
        }
        0x9B => {
            // RES 3, E
            cpu.registers.E = cpu.RES(3, cpu.registers.E);
            2
        }
        0x9C => {
            // RES 3, H
            cpu.registers.H = cpu.RES(3, cpu.registers.H);
            2
        }
        0x9D => {
            // RES 3, L
            cpu.registers.L = cpu.RES(3, cpu.registers.L);
            2
        }
        0x9E => {
            // RES 3, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.RES(3, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0x9F => {
            // RES 3, A
            cpu.registers.A = cpu.RES(3, cpu.registers.A);
            2
        }
        0xA0 => {
            // RES 4, B
            cpu.registers.B = cpu.RES(4, cpu.registers.B);
            2
        }
        0xA1 => {
            // RES 4, C
            cpu.registers.C = cpu.RES(4, cpu.registers.C);
            2
        }
        0xA2 => {
            // RES 4, D
            cpu.registers.D = cpu.RES(4, cpu.registers.D);
            2
        }
        0xA3 => {
            // RES 4, E
            cpu.registers.E = cpu.RES(4, cpu.registers.E);
            2
        }
        0xA4 => {
            // RES 4, H
            cpu.registers.H = cpu.RES(4, cpu.registers.H);
            2
        }
        0xA5 => {
            // RES 4, L
            cpu.registers.L = cpu.RES(4, cpu.registers.L);
            2
        }
        0xA6 => {
            // RES 4, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.RES(4, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0xA7 => {
            // RES 4, A
            cpu.registers.A = cpu.RES(4, cpu.registers.A);
            2
        }
        0xA8 => {
            // RES 5, B
            cpu.registers.B = cpu.RES(5, cpu.registers.B);
            2
        }
        0xA9 => {
            // RES 5, C
            cpu.registers.C = cpu.RES(5, cpu.registers.C);
            2
        }
        0xAA => {
            // RES 5, D
            cpu.registers.D = cpu.RES(5, cpu.registers.D);
            2
        }
        0xAB => {
            // RES 5, E
            cpu.registers.E = cpu.RES(5, cpu.registers.E);
            2
        }
        0xAC => {
            // RES 5, H
            cpu.registers.H = cpu.RES(5, cpu.registers.H);
            2
        }
        0xAD => {
            // RES 5, L
            cpu.registers.L = cpu.RES(5, cpu.registers.L);
            2
        }
        0xAE => {
            // RES 5, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.RES(5, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0xAF => {
            // RES 5, A
            cpu.registers.A = cpu.RES(5, cpu.registers.A);
            2
        }
        0xB0 => {
            // RES 6, B
            cpu.registers.B = cpu.RES(6, cpu.registers.B);
            2
        }
        0xB1 => {
            // RES 6, C
            cpu.registers.C = cpu.RES(6, cpu.registers.C);
            2
        }
        0xB2 => {
            // RES 6, D
            cpu.registers.D = cpu.RES(6, cpu.registers.D);
            2
        }
        0xB3 => {
            // RES 6, E
            cpu.registers.E = cpu.RES(6, cpu.registers.E);
            2
        }
        0xB4 => {
            // RES 6, H
            cpu.registers.H = cpu.RES(6, cpu.registers.H);
            2
        }
        0xB5 => {
            // RES 6, L
            cpu.registers.L = cpu.RES(6, cpu.registers.L);
            2
        }
        0xB6 => {
            // RES 6, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.RES(6, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0xB7 => {
            // RES 6, A
            cpu.registers.A = cpu.RES(6, cpu.registers.A);
            2
        }
        0xB8 => {
            // RES 7, B
            cpu.registers.B = cpu.RES(7, cpu.registers.B);
            2
        }
        0xB9 => {
            // RES 7, C
            cpu.registers.C = cpu.RES(7, cpu.registers.C);
            2
        }
        0xBA => {
            // RES 7, D
            cpu.registers.D = cpu.RES(7, cpu.registers.D);
            2
        }
        0xBB => {
            // RES 7, E
            cpu.registers.E = cpu.RES(7, cpu.registers.E);
            2
        }
        0xBC => {
            // RES 7, H
            cpu.registers.H = cpu.RES(7, cpu.registers.H);
            2
        }
        0xBD => {
            // RES 7, L
            cpu.registers.L = cpu.RES(7, cpu.registers.L);
            2
        }
        0xBE => {
            // RES 7, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.RES(7, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0xBF => {
            // RES 7, A
            cpu.registers.A = cpu.RES(7, cpu.registers.A);
            2
        }
        0xC0 => {
            // SET 0, B
            cpu.registers.B = cpu.SET(0, cpu.registers.B);
            2
        }
        0xC1 => {
            // SET 0, C
            cpu.registers.C = cpu.SET(0, cpu.registers.C);
            2
        }
        0xC2 => {
            // SET 0, D
            cpu.registers.D = cpu.SET(0, cpu.registers.D);
            2
        }
        0xC3 => {
            // SET 0, E
            cpu.registers.E = cpu.SET(0, cpu.registers.E);
            2
        }
        0xC4 => {
            // SET 0, H
            cpu.registers.H = cpu.SET(0, cpu.registers.H);
            2
        }
        0xC5 => {
            // SET 0, L
            cpu.registers.L = cpu.SET(0, cpu.registers.L);
            2
        }
        0xC6 => {
            // SET 0, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.SET(0, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0xC7 => {
            // SET 0, A
            cpu.registers.A = cpu.SET(0, cpu.registers.A);
            2
        }
        0xC8 => {
            // SET 1, B
            cpu.registers.B = cpu.SET(1, cpu.registers.B);
            2
        }
        0xC9 => {
            // SET 1, C
            cpu.registers.C = cpu.SET(1, cpu.registers.C);
            2
        }
        0xCA => {
            // SET 1, D
            cpu.registers.D = cpu.SET(1, cpu.registers.D);
            2
        }
        0xCB => {
            // SET 1, E
            cpu.registers.E = cpu.SET(1, cpu.registers.E);
            2
        }
        0xCC => {
            // SET 1, H
            cpu.registers.H = cpu.SET(1, cpu.registers.H);
            2
        }
        0xCD => {
            // SET 1, L
            cpu.registers.L = cpu.SET(1, cpu.registers.L);
            2
        }
        0xCE => {
            // SET 1, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.SET(1, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0xCF => {
            // SET 1, A
            cpu.registers.A = cpu.SET(1, cpu.registers.A);
            2
        }
        0xD0 => {
            // SET 2, B
            cpu.registers.B = cpu.SET(2, cpu.registers.B);
            2
        }
        0xD1 => {
            // SET 2, C
            cpu.registers.C = cpu.SET(2, cpu.registers.C);
            2
        }
        0xD2 => {
            // SET 2, D
            cpu.registers.D = cpu.SET(2, cpu.registers.D);
            2
        }
        0xD3 => {
            // SET 2, E
            cpu.registers.E = cpu.SET(2, cpu.registers.E);
            2
        }
        0xD4 => {
            // SET 2, H
            cpu.registers.H = cpu.SET(2, cpu.registers.H);
            2
        }
        0xD5 => {
            // SET 2, L
            cpu.registers.L = cpu.SET(2, cpu.registers.L);
            2
        }
        0xD6 => {
            // SET 2, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.SET(2, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0xD7 => {
            // SET 2, A
            cpu.registers.A = cpu.SET(2, cpu.registers.A);
            2
        }
        0xD8 => {
            // SET 3, B
            cpu.registers.B = cpu.SET(3, cpu.registers.B);
            2
        }
        0xD9 => {
            // SET 3, C
            cpu.registers.C = cpu.SET(3, cpu.registers.C);
            2
        }
        0xDA => {
            // SET 3, D
            cpu.registers.D = cpu.SET(3, cpu.registers.D);
            2
        }
        0xDB => {
            // SET 3, E
            cpu.registers.E = cpu.SET(3, cpu.registers.E);
            2
        }
        0xDC => {
            // SET 3, H
            cpu.registers.H = cpu.SET(3, cpu.registers.H);
            2
        }
        0xDD => {
            // SET 3, L
            cpu.registers.L = cpu.SET(3, cpu.registers.L);
            2
        }
        0xDE => {
            // SET 3, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.SET(3, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0xDF => {
            // SET 3, A
            cpu.registers.A = cpu.SET(3, cpu.registers.A);
            2
        }
        0xE0 => {
            // SET 4, B
            cpu.registers.B = cpu.SET(4, cpu.registers.B);
            2
        }
        0xE1 => {
            // SET 4, C
            cpu.registers.C = cpu.SET(4, cpu.registers.C);
            2
        }
        0xE2 => {
            // SET 4, D
            cpu.registers.D = cpu.SET(4, cpu.registers.D);
            2
        }
        0xE3 => {
            // SET 4, E
            cpu.registers.E = cpu.SET(4, cpu.registers.E);
            2
        }
        0xE4 => {
            // SET 4, H
            cpu.registers.H = cpu.SET(4, cpu.registers.H);
            2
        }
        0xE5 => {
            // SET 4, L
            cpu.registers.L = cpu.SET(4, cpu.registers.L);
            2
        }
        0xE6 => {
            // SET 4, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.SET(4, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0xE7 => {
            // SET 4, A
            cpu.registers.A = cpu.SET(4, cpu.registers.A);
            2
        }
        0xE8 => {
            // SET 5, B
            cpu.registers.B = cpu.SET(5, cpu.registers.B);
            2
        }
        0xE9 => {
            // SET 5, C
            cpu.registers.C = cpu.SET(5, cpu.registers.C);
            2
        }
        0xEA => {
            // SET 5, D
            cpu.registers.D = cpu.SET(5, cpu.registers.D);
            2
        }
        0xEB => {
            // SET 5, E
            cpu.registers.E = cpu.SET(5, cpu.registers.E);
            2
        }
        0xEC => {
            // SET 5, H
            cpu.registers.H = cpu.SET(5, cpu.registers.H);
            2
        }
        0xED => {
            // SET 5, L
            cpu.registers.L = cpu.SET(5, cpu.registers.L);
            2
        }
        0xEE => {
            // SET 5, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.SET(5, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0xEF => {
            // SET 5, A
            cpu.registers.A = cpu.SET(5, cpu.registers.A);
            2
        }
        0xF0 => {
            // SET 6, B
            cpu.registers.B = cpu.SET(6, cpu.registers.B);
            2
        }
        0xF1 => {
            // SET 6, C
            cpu.registers.C = cpu.SET(6, cpu.registers.C);
            2
        }
        0xF2 => {
            // SET 6, D
            cpu.registers.D = cpu.SET(6, cpu.registers.D);
            2
        }
        0xF3 => {
            // SET 6, E
            cpu.registers.E = cpu.SET(6, cpu.registers.E);
            2
        }
        0xF4 => {
            // SET 6, H
            cpu.registers.H = cpu.SET(6, cpu.registers.H);
            2
        }
        0xF5 => {
            // SET 6, L
            cpu.registers.L = cpu.SET(6, cpu.registers.L);
            2
        }
        0xF6 => {
            // SET 6, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.SET(6, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0xF7 => {
            // SET 6, A
            cpu.registers.A = cpu.SET(6, cpu.registers.A);
            2
        }
        0xF8 => {
            // SET 7, B
            cpu.registers.B = cpu.SET(7, cpu.registers.B);
            2
        }
        0xF9 => {
            // SET 7, C
            cpu.registers.C = cpu.SET(7, cpu.registers.C);
            2
        }
        0xFA => {
            // SET 7, D
            cpu.registers.D = cpu.SET(7, cpu.registers.D);
            2
        }
        0xFB => {
            // SET 7, E
            cpu.registers.E = cpu.SET(7, cpu.registers.E);
            2
        }
        0xFC => {
            // SET 7, H
            cpu.registers.H = cpu.SET(7, cpu.registers.H);
            2
        }
        0xFD => {
            // SET 7, L
            cpu.registers.L = cpu.SET(7, cpu.registers.L);
            2
        }
        0xFE => {
            // SET 7, (HL)
            let value = cpu.memory[cpu.get_hl() as usize];
            let result = cpu.SET(7, value);
            cpu.memory[cpu.get_hl() as usize] = result;
            4
        }
        0xFF => {
            // SET 7, A
            cpu.registers.A = cpu.SET(7, cpu.registers.A);
            2
        }
    }
}
