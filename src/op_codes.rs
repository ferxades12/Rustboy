use crate::{mmu::MMU, CPU};

pub fn execute_opcode(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let opcode = cpu.fetch_byte(mmu);
    //println!("opcode: {:02X}", opcode);
    match opcode {
        0x00 => 1,
        0x01 => {
            // LD BC, u16
            let value = cpu.fetch_word(mmu);
            cpu.set_bc(value);
            3
        }
        0x02 => {
            // LD (BC), A
            mmu.write_byte(cpu.get_bc(), cpu.registers.a);
            2
        }
        0x03 => {
            // inc BC
            let value = cpu.get_bc().wrapping_add(1);
            cpu.set_bc(value);
            2
        }
        0x04 => {
            // inc B
            cpu.registers.b = cpu.inc(cpu.registers.b);
            1
        }
        0x05 => {
            // dec B
            cpu.registers.b = cpu.dec(cpu.registers.b);
            1
        }
        0x06 => {
            // LD B, u8
            let value = cpu.fetch_byte(mmu);
            cpu.registers.b = value;
            2
        }
        0x07 => {
            // rlca
            cpu.rlca();
            1
        }
        0x08 => {
            // LD (u16), sp
            let word = cpu.fetch_word(mmu);
            mmu.write_word(word, cpu.registers.sp);
            5
        }
        0x09 => {
            // ADD HL, BC
            let result = cpu.add16(cpu.get_hl(), cpu.get_bc());
            cpu.set_hl(result);
            2
        }
        0x0A => {
            // LD A, (BC)
            cpu.registers.a = mmu.read_byte(cpu.get_bc());
            2
        }
        0x0B => {
            // dec BC
            let value = cpu.get_bc().wrapping_sub(1);
            cpu.set_bc(value);
            2
        }
        0x0C => {
            // inc C
            cpu.registers.c = cpu.inc(cpu.registers.c);
            1
        }
        0x0D => {
            // dec C
            cpu.registers.c = cpu.dec(cpu.registers.c);
            1
        }
        0x0E => {
            // LD C, u8
            let value = cpu.fetch_byte(mmu);
            cpu.registers.c = value;
            2
        }
        0x0F => {
            // rrca
            cpu.rrca();
            1
        }
        0x10 => {
            // STOP
            cpu.stop_flag = true;
            0
        }
        0x11 => {
            // LD DE, u16
            let value = cpu.fetch_word(mmu);
            cpu.set_de(value);
            3
        }
        0x12 => {
            // LD (DE), A
            mmu.write_byte(cpu.get_de(), cpu.registers.a);
            2
        }
        0x13 => {
            // inc DE
            let value = cpu.get_de().wrapping_add(1);
            cpu.set_de(value);
            2
        }
        0x14 => {
            // inc D
            cpu.registers.d = cpu.inc(cpu.registers.d);
            1
        }
        0x15 => {
            // dec D
            cpu.registers.d = cpu.dec(cpu.registers.d);
            1
        }
        0x16 => {
            // LD D, u8
            let value = cpu.fetch_byte(mmu);
            cpu.registers.d = value;
            2
        }
        0x17 => {
            // rla
            cpu.rla();
            1
        }
        0x18 => {
            // jr i8
            cpu.jr(true, mmu);
            3
        }
        0x19 => {
            // ADD HL, DE
            let result = cpu.add16(cpu.get_hl(), cpu.get_de());
            cpu.set_hl(result);
            2
        }
        0x1A => {
            // LD A, (DE)
            cpu.registers.a = mmu.read_byte(cpu.get_de());
            2
        }
        0x1B => {
            // dec DE
            let value = cpu.get_de().wrapping_sub(1);
            cpu.set_de(value);
            2
        }
        0x1C => {
            // inc E
            cpu.registers.e = cpu.inc(cpu.registers.e);
            1
        }
        0x1D => {
            // dec E
            cpu.registers.e = cpu.dec(cpu.registers.e);
            1
        }
        0x1E => {
            // LD E, u8
            let value = cpu.fetch_byte(mmu);
            cpu.registers.e = value;
            2
        }
        0x1F => {
            // rra
            cpu.rra();
            1
        }
        0x20 => {
            // jr NZ, i8
            let cond = !cpu.get_zf();
            cpu.jr(cond, mmu);
            if cond {
                3
            } else {
                2
            }
        }
        0x21 => {
            // LD HL, u16
            let value = cpu.fetch_word(mmu);
            cpu.set_hl(value);
            3
        }
        0x22 => {
            // LD (HL+), A
            let hl = cpu.get_hl();
            mmu.write_byte(hl, cpu.registers.a);
            cpu.set_hl(hl.wrapping_add(1));
            2
        }
        0x23 => {
            // inc HL
            let value = cpu.get_hl().wrapping_add(1);
            cpu.set_hl(value);
            2
        }
        0x24 => {
            // inc h
            cpu.registers.h = cpu.inc(cpu.registers.h);
            1
        }
        0x25 => {
            // dec h
            cpu.registers.h = cpu.dec(cpu.registers.h);
            1
        }
        0x26 => {
            // LD h, u8
            let value = cpu.fetch_byte(mmu);
            cpu.registers.h = value;
            2
        }
        0x27 => {
            // daa
            cpu.daa();
            1
        }
        0x28 => {
            // jr Z, i8
            let cond = cpu.get_zf();
            cpu.jr(cond, mmu);
            if cond {
                3
            } else {
                2
            }
        }
        0x29 => {
            // ADD HL, HL
            let result = cpu.add16(cpu.get_hl(), cpu.get_hl());
            cpu.set_hl(result);
            2
        }
        0x2A => {
            // LD A, (HL+)
            let hl = cpu.get_hl();
            cpu.registers.a = mmu.read_byte(hl);
            cpu.set_hl(hl.wrapping_add(1));
            2
        }
        0x2B => {
            // dec HL
            let value = cpu.get_hl().wrapping_sub(1);
            cpu.set_hl(value);
            2
        }
        0x2C => {
            // inc l
            cpu.registers.l = cpu.inc(cpu.registers.l);
            1
        }
        0x2D => {
            // dec l
            cpu.registers.l = cpu.dec(cpu.registers.l);
            1
        }
        0x2E => {
            // LD l, u8
            let value = cpu.fetch_byte(mmu);
            cpu.registers.l = value;
            2
        }
        0x2F => {
            // cpl
            cpu.cpl();
            1
        }
        0x30 => {
            // jr NC, i8
            let cond = !cpu.get_cf();
            cpu.jr(cond, mmu);
            if cond {
                3
            } else {
                2
            }
        }
        0x31 => {
            // LD sp, u16
            let value = cpu.fetch_word(mmu);
            cpu.registers.sp = value;
            3
        }
        0x32 => {
            // LD (HL-), A
            let hl = cpu.get_hl();
            mmu.write_byte(hl, cpu.registers.a);
            cpu.set_hl(hl.wrapping_sub(1));
            2
        }
        0x33 => {
            // inc sp
            let value = cpu.registers.sp.wrapping_add(1);
            cpu.registers.sp = value;
            2
        }
        0x34 => {
            // inc (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.inc(value);
            mmu.write_byte(cpu.get_hl(), result);
            3
        }
        0x35 => {
            // dec (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.dec(value);
            mmu.write_byte(cpu.get_hl(), result);
            3
        }
        0x36 => {
            // LD (HL), u8
            let value = cpu.fetch_byte(mmu);
            mmu.write_byte(cpu.get_hl(), value);
            3
        }
        0x37 => {
            // scf
            cpu.scf();
            1
        }
        0x38 => {
            // jr C, i8
            let cond = cpu.get_cf();
            cpu.jr(cond, mmu);

            if cond {
                3
            } else {
                2
            }
        }
        0x39 => {
            // ADD HL, sp
            let zf = cpu.get_zf();
            let result = cpu.add16(cpu.get_hl(), cpu.registers.sp);
            cpu.set_hl(result);
            cpu.set_zf(zf);
            2
        }
        0x3A => {
            // LD A, (HL-)
            let hl = cpu.get_hl();
            cpu.registers.a = mmu.read_byte(hl);
            cpu.set_hl(hl.wrapping_sub(1));
            2
        }
        0x3B => {
            // dec sp
            let value = cpu.registers.sp.wrapping_sub(1);
            cpu.registers.sp = value;
            2
        }
        0x3C => {
            // inc A
            cpu.registers.a = cpu.inc(cpu.registers.a);
            1
        }
        0x3D => {
            // dec A
            cpu.registers.a = cpu.dec(cpu.registers.a);
            1
        }
        0x3E => {
            // LD A, u8
            let value = cpu.fetch_byte(mmu);
            cpu.registers.a = value;
            2
        }
        0x3F => {
            // ccf
            cpu.ccf();
            1
        }
        0x40 => {
            // LD B, B
            cpu.registers.b = cpu.registers.b;
            1
        }
        0x41 => {
            // LD B, C
            cpu.registers.b = cpu.registers.c;
            1
        }
        0x42 => {
            // LD B, D
            cpu.registers.b = cpu.registers.d;
            1
        }
        0x43 => {
            // LD B, E
            cpu.registers.b = cpu.registers.e;
            1
        }
        0x44 => {
            // LD B, h
            cpu.registers.b = cpu.registers.h;
            1
        }
        0x45 => {
            // LD B, l
            cpu.registers.b = cpu.registers.l;
            1
        }
        0x46 => {
            // LD B, (HL)
            cpu.registers.b = mmu.read_byte(cpu.get_hl());
            2
        }
        0x47 => {
            // LD B, A
            cpu.registers.b = cpu.registers.a;
            1
        }
        0x48 => {
            // LD C, B
            cpu.registers.c = cpu.registers.b;
            1
        }
        0x49 => {
            // LD C, C
            cpu.registers.c = cpu.registers.c;
            1
        }
        0x4A => {
            // LD C, D
            cpu.registers.c = cpu.registers.d;
            1
        }
        0x4B => {
            // LD C, E
            cpu.registers.c = cpu.registers.e;
            1
        }
        0x4C => {
            // LD C, h
            cpu.registers.c = cpu.registers.h;
            1
        }
        0x4D => {
            // LD C, l
            cpu.registers.c = cpu.registers.l;
            1
        }
        0x4E => {
            // LD C, (HL)
            cpu.registers.c = mmu.read_byte(cpu.get_hl());
            2
        }
        0x4F => {
            // LD C, A
            cpu.registers.c = cpu.registers.a;
            1
        }
        0x50 => {
            // LD D, B
            cpu.registers.d = cpu.registers.b;
            1
        }
        0x51 => {
            // LD D, C
            cpu.registers.d = cpu.registers.c;
            1
        }
        0x52 => {
            // LD D, D
            cpu.registers.d = cpu.registers.d;
            1
        }
        0x53 => {
            // LD D, E
            cpu.registers.d = cpu.registers.e;
            1
        }
        0x54 => {
            // LD D, h
            cpu.registers.d = cpu.registers.h;
            1
        }
        0x55 => {
            // LD D, l
            cpu.registers.d = cpu.registers.l;
            1
        }
        0x56 => {
            // LD D, (HL)
            cpu.registers.d = mmu.read_byte(cpu.get_hl());
            2
        }
        0x57 => {
            // LD D, A
            cpu.registers.d = cpu.registers.a;
            1
        }
        0x58 => {
            // LD E, B
            cpu.registers.e = cpu.registers.b;
            1
        }
        0x59 => {
            // LD E, C
            cpu.registers.e = cpu.registers.c;
            1
        }
        0x5A => {
            // LD E, D
            cpu.registers.e = cpu.registers.d;
            1
        }
        0x5B => {
            // LD E, E
            cpu.registers.e = cpu.registers.e;
            1
        }
        0x5C => {
            // LD E, h
            cpu.registers.e = cpu.registers.h;
            1
        }
        0x5D => {
            // LD E, l
            cpu.registers.e = cpu.registers.l;
            1
        }
        0x5E => {
            // LD E, (HL)
            cpu.registers.e = mmu.read_byte(cpu.get_hl());
            2
        }
        0x5F => {
            // LD E, A
            cpu.registers.e = cpu.registers.a;
            1
        }
        0x60 => {
            // LD h, B
            cpu.registers.h = cpu.registers.b;
            1
        }
        0x61 => {
            // LD h, C
            cpu.registers.h = cpu.registers.c;
            1
        }
        0x62 => {
            // LD h, D
            cpu.registers.h = cpu.registers.d;
            1
        }
        0x63 => {
            // LD h, E
            cpu.registers.h = cpu.registers.e;
            1
        }
        0x64 => {
            // LD h, h
            cpu.registers.h = cpu.registers.h;
            1
        }
        0x65 => {
            // LD h, l
            cpu.registers.h = cpu.registers.l;
            1
        }
        0x66 => {
            // LD h, (HL)
            cpu.registers.h = mmu.read_byte(cpu.get_hl());
            2
        }
        0x67 => {
            // LD h, A
            cpu.registers.h = cpu.registers.a;
            1
        }
        0x68 => {
            // LD l, B
            cpu.registers.l = cpu.registers.b;
            1
        }
        0x69 => {
            // LD l, C
            cpu.registers.l = cpu.registers.c;
            1
        }
        0x6A => {
            // LD l, D
            cpu.registers.l = cpu.registers.d;
            1
        }
        0x6B => {
            // LD l, E
            cpu.registers.l = cpu.registers.e;
            1
        }
        0x6C => {
            // LD l, h
            cpu.registers.l = cpu.registers.h;
            1
        }
        0x6D => {
            // LD l, l
            cpu.registers.l = cpu.registers.l;
            1
        }
        0x6E => {
            // LD l, (HL)
            cpu.registers.l = mmu.read_byte(cpu.get_hl());
            2
        }
        0x6F => {
            // LD l, A
            cpu.registers.l = cpu.registers.a;
            1
        }
        0x70 => {
            // LD (HL), B
            mmu.write_byte(cpu.get_hl(), cpu.registers.b);
            2
        }
        0x71 => {
            // LD (HL), C
            mmu.write_byte(cpu.get_hl(), cpu.registers.c);
            2
        }
        0x72 => {
            // LD (HL), D
            mmu.write_byte(cpu.get_hl(), cpu.registers.d);
            2
        }
        0x73 => {
            // LD (HL), E
            mmu.write_byte(cpu.get_hl(), cpu.registers.e);
            2
        }
        0x74 => {
            // LD (HL), h
            mmu.write_byte(cpu.get_hl(), cpu.registers.h);
            2
        }
        0x75 => {
            // LD (HL), l
            mmu.write_byte(cpu.get_hl(), cpu.registers.l);
            2
        }
        0x76 => {
            // HALT
            cpu.halt_flag = true;
            0
        }
        0x77 => {
            // LD (HL), A
            mmu.write_byte(cpu.get_hl(), cpu.registers.a);
            2
        }
        0x78 => {
            // LD A, B
            cpu.registers.a = cpu.registers.b;
            1
        }
        0x79 => {
            // LD A, C
            cpu.registers.a = cpu.registers.c;
            1
        }
        0x7A => {
            // LD A, D
            cpu.registers.a = cpu.registers.d;
            1
        }
        0x7B => {
            // LD A, E
            cpu.registers.a = cpu.registers.e;
            1
        }
        0x7C => {
            // LD A, h
            cpu.registers.a = cpu.registers.h;
            1
        }
        0x7D => {
            // LD A, l
            cpu.registers.a = cpu.registers.l;
            1
        }
        0x7E => {
            // LD A, (HL)
            cpu.registers.a = mmu.read_byte(cpu.get_hl());
            2
        }
        0x7F => {
            // LD A, A
            cpu.registers.a = cpu.registers.a;
            1
        }
        0x80 => {
            // ADD A, B
            cpu.registers.a = cpu.add8(cpu.registers.b);
            1
        }
        0x81 => {
            // ADD A, C
            cpu.registers.a = cpu.add8(cpu.registers.c);
            1
        }
        0x82 => {
            // ADD A, D
            cpu.registers.a = cpu.add8(cpu.registers.d);
            1
        }
        0x83 => {
            // ADD A, E
            cpu.registers.a = cpu.add8(cpu.registers.e);
            1
        }
        0x84 => {
            // ADD A, h
            cpu.registers.a = cpu.add8(cpu.registers.h);
            1
        }
        0x85 => {
            // ADD A, l
            cpu.registers.a = cpu.add8(cpu.registers.l);
            1
        }
        0x86 => {
            // ADD A, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.registers.a = cpu.add8(value);
            2
        }
        0x87 => {
            // ADD A, A
            cpu.registers.a = cpu.add8(cpu.registers.a);
            1
        }
        0x88 => {
            // adc A, B
            cpu.registers.a = cpu.adc(cpu.registers.b);
            2
        }
        0x89 => {
            // adc A, C
            cpu.registers.a = cpu.adc(cpu.registers.c);
            1
        }
        0x8A => {
            // adc A, D
            cpu.registers.a = cpu.adc(cpu.registers.d);
            1
        }
        0x8B => {
            // adc A, E
            cpu.registers.a = cpu.adc(cpu.registers.e);
            1
        }
        0x8C => {
            // adc A, h
            cpu.registers.a = cpu.adc(cpu.registers.h);
            1
        }
        0x8D => {
            // adc A, l
            cpu.registers.a = cpu.adc(cpu.registers.l);
            1
        }
        0x8E => {
            // adc A, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.registers.a = cpu.adc(value);
            2
        }
        0x8F => {
            // adc A, A
            cpu.registers.a = cpu.adc(cpu.registers.a);
            1
        }
        0x90 => {
            // sub A, B
            cpu.registers.a = cpu.sub(cpu.registers.b);
            1
        }
        0x91 => {
            // sub A, C
            cpu.registers.a = cpu.sub(cpu.registers.c);
            1
        }
        0x92 => {
            // sub A, D
            cpu.registers.a = cpu.sub(cpu.registers.d);
            1
        }
        0x93 => {
            // sub A, E
            cpu.registers.a = cpu.sub(cpu.registers.e);
            1
        }
        0x94 => {
            // sub A, h
            cpu.registers.a = cpu.sub(cpu.registers.h);
            1
        }
        0x95 => {
            // sub A, l
            cpu.registers.a = cpu.sub(cpu.registers.l);
            1
        }
        0x96 => {
            // sub A, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.registers.a = cpu.sub(value);
            2
        }
        0x97 => {
            // sub A, A
            cpu.registers.a = cpu.sub(cpu.registers.a);
            1
        }
        0x98 => {
            // sbc A, B
            cpu.registers.a = cpu.sbc(cpu.registers.b);
            1
        }
        0x99 => {
            // sbc A, C
            cpu.registers.a = cpu.sbc(cpu.registers.c);
            1
        }
        0x9A => {
            // sbc A, D
            cpu.registers.a = cpu.sbc(cpu.registers.d);
            1
        }
        0x9B => {
            // sbc A, E
            cpu.registers.a = cpu.sbc(cpu.registers.e);
            1
        }
        0x9C => {
            // sbc A, h
            cpu.registers.a = cpu.sbc(cpu.registers.h);
            1
        }
        0x9D => {
            // sbc A, l
            cpu.registers.a = cpu.sbc(cpu.registers.l);
            1
        }
        0x9E => {
            // sbc A, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.registers.a = cpu.sbc(value);
            2
        }
        0x9F => {
            // sbc A, A
            cpu.registers.a = cpu.sbc(cpu.registers.a);
            1
        }
        0xA0 => {
            // and A, B
            cpu.and(cpu.registers.b);
            1
        }
        0xA1 => {
            // and A, C
            cpu.and(cpu.registers.c);
            1
        }
        0xA2 => {
            // and A, D
            cpu.and(cpu.registers.d);
            1
        }
        0xA3 => {
            // and A, E
            cpu.and(cpu.registers.e);
            1
        }
        0xA4 => {
            // and A, h
            cpu.and(cpu.registers.h);
            1
        }
        0xA5 => {
            // and A, l
            cpu.and(cpu.registers.l);
            1
        }
        0xA6 => {
            // and A, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.and(value);
            2
        }
        0xA7 => {
            // and A, A
            cpu.and(cpu.registers.a);
            1
        }
        0xA8 => {
            // xor A, B
            cpu.xor(cpu.registers.b);
            1
        }
        0xA9 => {
            // xor A, C
            cpu.xor(cpu.registers.c);
            1
        }
        0xAA => {
            // xor A, D
            cpu.xor(cpu.registers.d);
            1
        }
        0xAB => {
            // xor A, E
            cpu.xor(cpu.registers.e);
            1
        }
        0xAC => {
            // xor A, h
            cpu.xor(cpu.registers.h);
            1
        }
        0xAD => {
            // xor A, l
            cpu.xor(cpu.registers.l);
            1
        }
        0xAE => {
            // xor A, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.xor(value);
            2
        }
        0xAF => {
            // xor A, A
            cpu.xor(cpu.registers.a);
            1
        }
        0xB0 => {
            // or A, B
            cpu.or(cpu.registers.b);
            1
        }
        0xB1 => {
            // or A, C
            cpu.or(cpu.registers.c);
            1
        }
        0xB2 => {
            // or A, D
            cpu.or(cpu.registers.d);
            1
        }
        0xB3 => {
            // or A, E
            cpu.or(cpu.registers.e);
            1
        }
        0xB4 => {
            // or A, h
            cpu.or(cpu.registers.h);
            1
        }
        0xB5 => {
            // or A, l
            cpu.or(cpu.registers.l);
            1
        }
        0xB6 => {
            // or A, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.or(value);
            2
        }
        0xB7 => {
            // or A, A
            cpu.or(cpu.registers.a);
            1
        }
        0xB8 => {
            // cp A, B
            cpu.cp(cpu.registers.b);
            1
        }
        0xB9 => {
            // cp A, C
            cpu.cp(cpu.registers.c);
            1
        }
        0xBA => {
            // cp A, D
            cpu.cp(cpu.registers.d);
            1
        }
        0xBB => {
            // cp A, E
            cpu.cp(cpu.registers.e);
            1
        }
        0xBC => {
            // cp A, h
            cpu.cp(cpu.registers.h);
            1
        }
        0xBD => {
            // cp A, l
            cpu.cp(cpu.registers.l);
            1
        }
        0xBE => {
            // cp A, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.cp(value);
            2
        }
        0xBF => {
            // cp A, A
            cpu.cp(cpu.registers.a);
            1
        }
        0xC0 => {
            // ret NZ
            let cond = !cpu.get_zf();
            cpu.ret(cond, mmu);

            if cond {
                5
            } else {
                2
            }
        }
        0xC1 => {
            // pop BC
            let value = cpu.pop(mmu);
            cpu.set_bc(value);
            3
        }
        0xC2 => {
            // jp NZ, u16
            let cond = !cpu.get_zf();
            cpu.jp(cond, mmu);
            if cond {
                4
            } else {
                3
            }
        }
        0xC3 => {
            // jp u16
            cpu.jp(true, mmu);
            4
        }
        0xC4 => {
            // call NZ, u16
            let cond = !cpu.get_zf();
            cpu.call(cond, mmu);
            if cond {
                6
            } else {
                3
            }
        }
        0xC5 => {
            // push BC
            cpu.push(cpu.get_bc(), mmu);
            4
        }
        0xC6 => {
            // ADD A, u8
            let value = cpu.fetch_byte(mmu);
            cpu.registers.a = cpu.add8(value);
            2
        }
        0xC7 => {
            // rst 00H
            cpu.rst(0x00, mmu);
            4
        }
        0xC8 => {
            // ret Z
            let cond = cpu.get_zf();
            cpu.ret(cond, mmu);
            if cond {
                5
            } else {
                2
            }
        }
        0xC9 => {
            // ret
            cpu.ret(true, mmu);
            4
        }
        0xCA => {
            // jp Z, u16
            let cond = cpu.get_zf();
            cpu.jp(cond, mmu);
            if cond {
                4
            } else {
                3
            }
        }
        0xCB => {
            // PREFIX CB
            execute_cb_opcode(cpu, mmu)
        }
        0xCC => {
            // call Z, u16
            let cond = cpu.get_zf();
            cpu.call(cond, mmu);
            if cond {
                6
            } else {
                3
            }
        }
        0xCD => {
            // call u16
            cpu.call(true, mmu);
            6
        }
        0xCE => {
            // adc A, u8
            let value = cpu.fetch_byte(mmu);
            cpu.registers.a = cpu.adc(value);
            2
        }
        0xCF => {
            // rst 08H
            cpu.rst(0x08, mmu);
            4
        }
        0xD0 => {
            // ret NC
            let cond = !cpu.get_cf();
            cpu.ret(cond, mmu);
            if cond {
                5
            } else {
                2
            }
        }
        0xD1 => {
            // pop DE
            let value = cpu.pop(mmu);
            cpu.set_de(value);
            3
        }
        0xD2 => {
            // jp NC, u16
            let cond = !cpu.get_cf();
            cpu.jp(cond, mmu);
            if cond {
                4
            } else {
                3
            }
        }
        0xD4 => {
            // call NC, u16
            let cond = !cpu.get_cf();
            cpu.call(cond, mmu);
            if cond {
                6
            } else {
                3
            }
        }
        0xD5 => {
            // push DE
            cpu.push(cpu.get_de(), mmu);
            4
        }
        0xD6 => {
            // sub A, u8
            let value = cpu.fetch_byte(mmu);
            cpu.registers.a = cpu.sub(value);
            2
        }
        0xD7 => {
            // rst 10H
            cpu.rst(0x10, mmu);
            4
        }
        0xD8 => {
            // ret C
            let cond = cpu.get_cf();
            cpu.ret(cond, mmu);
            if cond {
                5
            } else {
                2
            }
        }
        0xD9 => {
            // RETI
            cpu.ret(true, mmu);
            cpu.ei_flag = true;
            4
        }
        0xDA => {
            // jp C, u16
            let cond = cpu.get_cf();
            cpu.jp(cond, mmu);
            if cond {
                4
            } else {
                3
            }
        }
        0xDC => {
            // call C, u16
            let cond = cpu.get_cf();
            cpu.call(cond, mmu);
            if cond {
                6
            } else {
                3
            }
        }
        0xDE => {
            // sbc A, u8
            let value = cpu.fetch_byte(mmu);
            cpu.registers.a = cpu.sbc(value);
            2
        }
        0xDF => {
            // rst 18H
            cpu.rst(0x18, mmu);
            4
        }
        0xE0 => {
            // LD (FF00 + u8), A
            let value = cpu.fetch_byte(mmu);
            mmu.write_byte(0xFF00 + value as u16, cpu.registers.a);
            3
        }
        0xE1 => {
            // pop HL
            let value = cpu.pop(mmu);
            cpu.set_hl(value);
            3
        }
        0xE2 => {
            // LD (FF00 + C), A
            mmu.write_byte(0xFF00 + cpu.registers.c as u16, cpu.registers.a);
            2
        }
        0xE5 => {
            // push HL
            cpu.push(cpu.get_hl(), mmu);
            4
        }
        0xE6 => {
            // and A, u8
            let value = cpu.fetch_byte(mmu);
            cpu.and(value);
            2
        }
        0xE7 => {
            // rst 20H
            cpu.rst(0x20, mmu);
            4
        }
        0xE8 => {
            // ADD sp, i8
            let value = cpu.fetch_byte(mmu) as i8 as i16; // Convertir a i16 para la suma correcta
            let sp = cpu.registers.sp as i16;
            let result = sp.wrapping_add(value) as u16;

            // Calcular los flags de carry y half-carry
            let carry = ((sp & 0xFF) + (value & 0xFF)) > 0xFF;
            let half_carry = ((sp & 0xF) + (value & 0xF)) > 0xF;

            // Actualizar los flags
            cpu.update_flags(false, carry, half_carry, false);

            // Actualizar el registro sp
            cpu.registers.sp = result;
            4
        }
        0xE9 => {
            // jp HL
            cpu.registers.pc = cpu.get_hl();
            1
        }
        0xEA => {
            // LD (u16), A
            let value = cpu.fetch_word(mmu);
            mmu.write_byte(value, cpu.registers.a);
            4
        }
        0xEE => {
            // xor A, u8
            let value = cpu.fetch_byte(mmu);
            cpu.xor(value);
            2
        }
        0xEF => {
            // rst 28H
            cpu.rst(0x28, mmu);
            4
        }
        0xF0 => {
            // LD A, (FF00 + u8)
            let value = cpu.fetch_byte(mmu);
            cpu.registers.a = mmu.read_byte(0xFF00 + value as u16);
            3
        }
        0xF1 => {
            // pop AF
            let value = cpu.pop(mmu) & 0xFFF0;
            cpu.set_af(value);
            3
        }
        0xF2 => {
            // LD A, (FF00 + C)
            cpu.registers.a = mmu.read_byte(0xFF00 + cpu.registers.c as u16);
            2
        }
        0xF3 => {
            // Disable Interrupt
            cpu.ei_flag = false;
            cpu.ime = false;
            1
        }
        0xF5 => {
            // push AF
            cpu.push(cpu.get_af(), mmu);
            4
        }
        0xF6 => {
            // or A, u8
            let value = cpu.fetch_byte(mmu);
            cpu.or(value);
            2
        }
        0xF7 => {
            // rst 30H
            cpu.rst(0x30, mmu);
            4
        }
        0xF8 => {
            // LD HL, sp+i8
            let value = cpu.fetch_byte(mmu) as i8 as i16;
            let sp = cpu.registers.sp as i16;
            let result = sp.wrapping_add(value) as u16;

            // Calcular los flags de carry y half-carry
            let carry = ((sp & 0xFF) + (value & 0xFF)) > 0xFF;
            let half_carry = ((sp & 0xF) + (value & 0xF)) > 0xF;

            // Actualizar los flags
            cpu.update_flags(false, carry, half_carry, false);

            // Actualizar el registro HL
            cpu.set_hl(result);
            3
        }
        0xF9 => {
            // LD sp, HL
            cpu.registers.sp = cpu.get_hl();
            2
        }
        0xFA => {
            // LD A, (u16)
            let value = cpu.fetch_word(mmu);
            cpu.registers.a = mmu.read_byte(value);
            4
        }
        0xFB => {
            // Enable Interrupt
            cpu.ei_flag = true;
            1
        }
        0xFE => {
            // cp A, u8
            let value = cpu.fetch_byte(mmu);
            cpu.cp(value);
            2
        }
        0xFF => {
            // rst 38H
            cpu.rst(0x38, mmu);
            4
        }
        _ => panic!("Unknown opcode: 0x{:X}", opcode),
    }
}

fn execute_cb_opcode(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let op_code = cpu.fetch_byte(mmu);
    match op_code {
        0x00 => {
            // rlc B
            cpu.registers.b = cpu.rlc(cpu.registers.b);
            2
        }
        0x01 => {
            // rlc C
            cpu.registers.c = cpu.rlc(cpu.registers.c);
            2
        }
        0x02 => {
            // rlc D
            cpu.registers.d = cpu.rlc(cpu.registers.d);
            2
        }
        0x03 => {
            // rlc E
            cpu.registers.e = cpu.rlc(cpu.registers.e);
            2
        }
        0x04 => {
            // rlc h
            cpu.registers.h = cpu.rlc(cpu.registers.h);
            2
        }
        0x05 => {
            // rlc l
            cpu.registers.l = cpu.rlc(cpu.registers.l);
            2
        }
        0x06 => {
            // rlc (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.rlc(value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0x07 => {
            // rlc A
            cpu.registers.a = cpu.rlc(cpu.registers.a);
            2
        }
        0x08 => {
            // rrc B
            cpu.registers.b = cpu.rrc(cpu.registers.b);
            2
        }
        0x09 => {
            // rrc C
            cpu.registers.c = cpu.rrc(cpu.registers.c);
            2
        }
        0x0A => {
            // rrc D
            cpu.registers.d = cpu.rrc(cpu.registers.d);
            2
        }
        0x0B => {
            // rrc E
            cpu.registers.e = cpu.rrc(cpu.registers.e);
            2
        }
        0x0C => {
            // rrc h
            cpu.registers.h = cpu.rrc(cpu.registers.h);
            2
        }
        0x0D => {
            // rrc l
            cpu.registers.l = cpu.rrc(cpu.registers.l);
            2
        }
        0x0E => {
            // rrc (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.rrc(value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0x0F => {
            // rrc A
            cpu.registers.a = cpu.rrc(cpu.registers.a);
            2
        }
        0x10 => {
            // rl B
            cpu.registers.b = cpu.rl(cpu.registers.b);
            2
        }
        0x11 => {
            // rl C
            cpu.registers.c = cpu.rl(cpu.registers.c);
            2
        }
        0x12 => {
            // rl D
            cpu.registers.d = cpu.rl(cpu.registers.d);
            2
        }
        0x13 => {
            // rl E
            cpu.registers.e = cpu.rl(cpu.registers.e);
            2
        }
        0x14 => {
            // rl h
            cpu.registers.h = cpu.rl(cpu.registers.h);
            2
        }
        0x15 => {
            // rl l
            cpu.registers.l = cpu.rl(cpu.registers.l);
            2
        }
        0x16 => {
            // rl (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.rl(value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0x17 => {
            // rl A
            cpu.registers.a = cpu.rl(cpu.registers.a);
            2
        }
        0x18 => {
            // rr B
            cpu.registers.b = cpu.rr(cpu.registers.b);
            2
        }
        0x19 => {
            // rr C
            cpu.registers.c = cpu.rr(cpu.registers.c);
            2
        }
        0x1A => {
            // rr D
            cpu.registers.d = cpu.rr(cpu.registers.d);
            2
        }
        0x1B => {
            // rr E
            cpu.registers.e = cpu.rr(cpu.registers.e);
            2
        }
        0x1C => {
            // rr h
            cpu.registers.h = cpu.rr(cpu.registers.h);
            2
        }
        0x1D => {
            // rr l
            cpu.registers.l = cpu.rr(cpu.registers.l);
            2
        }
        0x1E => {
            // rr (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.rr(value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0x1F => {
            // rr A
            cpu.registers.a = cpu.rr(cpu.registers.a);
            2
        }
        0x20 => {
            // sla B
            cpu.registers.b = cpu.sla(cpu.registers.b);
            2
        }
        0x21 => {
            // sla C
            cpu.registers.c = cpu.sla(cpu.registers.c);
            2
        }
        0x22 => {
            // sla D
            cpu.registers.d = cpu.sla(cpu.registers.d);
            2
        }
        0x23 => {
            // sla E
            cpu.registers.e = cpu.sla(cpu.registers.e);
            2
        }
        0x24 => {
            // sla h
            cpu.registers.h = cpu.sla(cpu.registers.h);
            2
        }
        0x25 => {
            // sla l
            cpu.registers.l = cpu.sla(cpu.registers.l);
            2
        }
        0x26 => {
            // sla (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.sla(value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0x27 => {
            // sla A
            cpu.registers.a = cpu.sla(cpu.registers.a);
            2
        }
        0x28 => {
            // sra B
            cpu.registers.b = cpu.sra(cpu.registers.b);
            2
        }
        0x29 => {
            // sra C
            cpu.registers.c = cpu.sra(cpu.registers.c);
            2
        }
        0x2A => {
            // sra D
            cpu.registers.d = cpu.sra(cpu.registers.d);
            2
        }
        0x2B => {
            // sra E
            cpu.registers.e = cpu.sra(cpu.registers.e);
            2
        }
        0x2C => {
            // sra h
            cpu.registers.h = cpu.sra(cpu.registers.h);
            2
        }
        0x2D => {
            // sra l
            cpu.registers.l = cpu.sra(cpu.registers.l);
            2
        }
        0x2E => {
            // sra (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.sra(value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0x2F => {
            // sra A
            cpu.registers.a = cpu.sra(cpu.registers.a);
            2
        }
        0x30 => {
            // swap B
            cpu.registers.b = cpu.swap(cpu.registers.b);
            2
        }
        0x31 => {
            // swap C
            cpu.registers.c = cpu.swap(cpu.registers.c);
            2
        }
        0x32 => {
            // swap D
            cpu.registers.d = cpu.swap(cpu.registers.d);
            2
        }
        0x33 => {
            // swap E
            cpu.registers.e = cpu.swap(cpu.registers.e);
            2
        }
        0x34 => {
            // swap h
            cpu.registers.h = cpu.swap(cpu.registers.h);
            2
        }
        0x35 => {
            // swap l
            cpu.registers.l = cpu.swap(cpu.registers.l);
            2
        }
        0x36 => {
            // swap (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.swap(value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0x37 => {
            // swap A
            cpu.registers.a = cpu.swap(cpu.registers.a);
            2
        }
        0x38 => {
            // srl B
            cpu.registers.b = cpu.srl(cpu.registers.b);
            2
        }
        0x39 => {
            // srl C
            cpu.registers.c = cpu.srl(cpu.registers.c);
            2
        }
        0x3A => {
            // srl D
            cpu.registers.d = cpu.srl(cpu.registers.d);
            2
        }
        0x3B => {
            // srl E
            cpu.registers.e = cpu.srl(cpu.registers.e);
            2
        }
        0x3C => {
            // srl h
            cpu.registers.h = cpu.srl(cpu.registers.h);
            2
        }
        0x3D => {
            // srl l
            cpu.registers.l = cpu.srl(cpu.registers.l);
            2
        }
        0x3E => {
            // srl (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.srl(value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0x3F => {
            // srl A
            cpu.registers.a = cpu.srl(cpu.registers.a);
            2
        }
        0x40 => {
            // bit 0, B
            cpu.bit(0, cpu.registers.b);
            2
        }
        0x41 => {
            // bit 0, C
            cpu.bit(0, cpu.registers.c);
            2
        }
        0x42 => {
            // bit 0, D
            cpu.bit(0, cpu.registers.d);
            2
        }
        0x43 => {
            // bit 0, E
            cpu.bit(0, cpu.registers.e);
            2
        }
        0x44 => {
            // bit 0, h
            cpu.bit(0, cpu.registers.h);
            2
        }
        0x45 => {
            // bit 0, l
            cpu.bit(0, cpu.registers.l);
            2
        }
        0x46 => {
            // bit 0, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.bit(0, value);
            3
        }
        0x47 => {
            // bit 0, A
            cpu.bit(0, cpu.registers.a);
            2
        }
        0x48 => {
            // bit 1, B
            cpu.bit(1, cpu.registers.b);
            2
        }
        0x49 => {
            // bit 1, C
            cpu.bit(1, cpu.registers.c);
            2
        }
        0x4A => {
            // bit 1, D
            cpu.bit(1, cpu.registers.d);
            2
        }
        0x4B => {
            // bit 1, E
            cpu.bit(1, cpu.registers.e);
            2
        }
        0x4C => {
            // bit 1, h
            cpu.bit(1, cpu.registers.h);
            2
        }
        0x4D => {
            // bit 1, l
            cpu.bit(1, cpu.registers.l);
            2
        }
        0x4E => {
            // bit 1, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.bit(1, value);
            3
        }
        0x4F => {
            // bit 1, A
            cpu.bit(1, cpu.registers.a);
            2
        }
        0x50 => {
            // bit 2, B
            cpu.bit(2, cpu.registers.b);
            2
        }
        0x51 => {
            // bit 2, C
            cpu.bit(2, cpu.registers.c);
            2
        }
        0x52 => {
            // bit 2, D
            cpu.bit(2, cpu.registers.d);
            2
        }
        0x53 => {
            // bit 2, E
            cpu.bit(2, cpu.registers.e);
            2
        }
        0x54 => {
            // bit 2, h
            cpu.bit(2, cpu.registers.h);
            2
        }
        0x55 => {
            // bit 2, l
            cpu.bit(2, cpu.registers.l);
            2
        }
        0x56 => {
            // bit 2, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.bit(2, value);
            3
        }
        0x57 => {
            // bit 2, A
            cpu.bit(2, cpu.registers.a);
            2
        }
        0x58 => {
            // bit 3, B
            cpu.bit(3, cpu.registers.b);
            2
        }
        0x59 => {
            // bit 3, C
            cpu.bit(3, cpu.registers.c);
            2
        }
        0x5A => {
            // bit 3, D
            cpu.bit(3, cpu.registers.d);
            2
        }
        0x5B => {
            // bit 3, E
            cpu.bit(3, cpu.registers.e);
            2
        }
        0x5C => {
            // bit 3, h
            cpu.bit(3, cpu.registers.h);
            2
        }
        0x5D => {
            // bit 3, l
            cpu.bit(3, cpu.registers.l);
            2
        }
        0x5E => {
            // bit 3, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.bit(3, value);
            3
        }
        0x5F => {
            // bit 3, A
            cpu.bit(3, cpu.registers.a);
            2
        }
        0x60 => {
            // bit 4, B
            cpu.bit(4, cpu.registers.b);
            2
        }
        0x61 => {
            // bit 4, C
            cpu.bit(4, cpu.registers.c);
            2
        }
        0x62 => {
            // bit 4, D
            cpu.bit(4, cpu.registers.d);
            2
        }
        0x63 => {
            // bit 4, E
            cpu.bit(4, cpu.registers.e);
            2
        }
        0x64 => {
            // bit 4, h
            cpu.bit(4, cpu.registers.h);
            2
        }
        0x65 => {
            // bit 4, l
            cpu.bit(4, cpu.registers.l);
            2
        }
        0x66 => {
            // bit 4, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.bit(4, value);
            3
        }
        0x67 => {
            // bit 4, A
            cpu.bit(4, cpu.registers.a);
            2
        }
        0x68 => {
            // bit 5, B
            cpu.bit(5, cpu.registers.b);
            2
        }
        0x69 => {
            // bit 5, C
            cpu.bit(5, cpu.registers.c);
            2
        }
        0x6A => {
            // bit 5, D
            cpu.bit(5, cpu.registers.d);
            2
        }
        0x6B => {
            // bit 5, E
            cpu.bit(5, cpu.registers.e);
            2
        }
        0x6C => {
            // bit 5, h
            cpu.bit(5, cpu.registers.h);
            2
        }
        0x6D => {
            // bit 5, l
            cpu.bit(5, cpu.registers.l);
            2
        }
        0x6E => {
            // bit 5, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.bit(5, value);
            3
        }
        0x6F => {
            // bit 5, A
            cpu.bit(5, cpu.registers.a);
            2
        }
        0x70 => {
            // bit 6, B
            cpu.bit(6, cpu.registers.b);
            2
        }
        0x71 => {
            // bit 6, C
            cpu.bit(6, cpu.registers.c);
            2
        }
        0x72 => {
            // bit 6, D
            cpu.bit(6, cpu.registers.d);
            2
        }
        0x73 => {
            // bit 6, E
            cpu.bit(6, cpu.registers.e);
            2
        }
        0x74 => {
            // bit 6, h
            cpu.bit(6, cpu.registers.h);
            2
        }
        0x75 => {
            // bit 6, l
            cpu.bit(6, cpu.registers.l);
            2
        }
        0x76 => {
            // bit 6, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.bit(6, value);
            3
        }
        0x77 => {
            // bit 6, A
            cpu.bit(6, cpu.registers.a);
            2
        }
        0x78 => {
            // bit 7, B
            cpu.bit(7, cpu.registers.b);
            2
        }
        0x79 => {
            // bit 7, C
            cpu.bit(7, cpu.registers.c);
            2
        }
        0x7A => {
            // bit 7, D
            cpu.bit(7, cpu.registers.d);
            2
        }
        0x7B => {
            // bit 7, E
            cpu.bit(7, cpu.registers.e);
            2
        }
        0x7C => {
            // bit 7, h
            cpu.bit(7, cpu.registers.h);
            2
        }
        0x7D => {
            // bit 7, l
            cpu.bit(7, cpu.registers.l);
            2
        }
        0x7E => {
            // bit 7, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            cpu.bit(7, value);
            3
        }
        0x7F => {
            // bit 7, A
            cpu.bit(7, cpu.registers.a);
            2
        }
        0x80 => {
            // res 0, B
            cpu.registers.b = cpu.res(0, cpu.registers.b);
            2
        }
        0x81 => {
            // res 0, C
            cpu.registers.c = cpu.res(0, cpu.registers.c);
            2
        }
        0x82 => {
            // res 0, D
            cpu.registers.d = cpu.res(0, cpu.registers.d);
            2
        }
        0x83 => {
            // res 0, E
            cpu.registers.e = cpu.res(0, cpu.registers.e);
            2
        }
        0x84 => {
            // res 0, h
            cpu.registers.h = cpu.res(0, cpu.registers.h);
            2
        }
        0x85 => {
            // res 0, l
            cpu.registers.l = cpu.res(0, cpu.registers.l);
            2
        }
        0x86 => {
            // res 0, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.res(0, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0x87 => {
            // res 0, A
            cpu.registers.a = cpu.res(0, cpu.registers.a);
            2
        }
        0x88 => {
            // res 1, B
            cpu.registers.b = cpu.res(1, cpu.registers.b);
            2
        }
        0x89 => {
            // res 1, C
            cpu.registers.c = cpu.res(1, cpu.registers.c);
            2
        }
        0x8A => {
            // res 1, D
            cpu.registers.d = cpu.res(1, cpu.registers.d);
            2
        }
        0x8B => {
            // res 1, E
            cpu.registers.e = cpu.res(1, cpu.registers.e);
            2
        }
        0x8C => {
            // res 1, h
            cpu.registers.h = cpu.res(1, cpu.registers.h);
            2
        }
        0x8D => {
            // res 1, l
            cpu.registers.l = cpu.res(1, cpu.registers.l);
            2
        }
        0x8E => {
            // res 1, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.res(1, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0x8F => {
            // res 1, A
            cpu.registers.a = cpu.res(1, cpu.registers.a);
            2
        }
        0x90 => {
            // res 2, B
            cpu.registers.b = cpu.res(2, cpu.registers.b);
            2
        }
        0x91 => {
            // res 2, C
            cpu.registers.c = cpu.res(2, cpu.registers.c);
            2
        }
        0x92 => {
            // res 2, D
            cpu.registers.d = cpu.res(2, cpu.registers.d);
            2
        }
        0x93 => {
            // res 2, E
            cpu.registers.e = cpu.res(2, cpu.registers.e);
            2
        }
        0x94 => {
            // res 2, h
            cpu.registers.h = cpu.res(2, cpu.registers.h);
            2
        }
        0x95 => {
            // res 2, l
            cpu.registers.l = cpu.res(2, cpu.registers.l);
            2
        }
        0x96 => {
            // res 2, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.res(2, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0x97 => {
            // res 2, A
            cpu.registers.a = cpu.res(2, cpu.registers.a);
            2
        }
        0x98 => {
            // res 3, B
            cpu.registers.b = cpu.res(3, cpu.registers.b);
            2
        }
        0x99 => {
            // res 3, C
            cpu.registers.c = cpu.res(3, cpu.registers.c);
            2
        }
        0x9A => {
            // res 3, D
            cpu.registers.d = cpu.res(3, cpu.registers.d);
            2
        }
        0x9B => {
            // res 3, E
            cpu.registers.e = cpu.res(3, cpu.registers.e);
            2
        }
        0x9C => {
            // res 3, h
            cpu.registers.h = cpu.res(3, cpu.registers.h);
            2
        }
        0x9D => {
            // res 3, l
            cpu.registers.l = cpu.res(3, cpu.registers.l);
            2
        }
        0x9E => {
            // res 3, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.res(3, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0x9F => {
            // res 3, A
            cpu.registers.a = cpu.res(3, cpu.registers.a);
            2
        }
        0xA0 => {
            // res 4, B
            cpu.registers.b = cpu.res(4, cpu.registers.b);
            2
        }
        0xA1 => {
            // res 4, C
            cpu.registers.c = cpu.res(4, cpu.registers.c);
            2
        }
        0xA2 => {
            // res 4, D
            cpu.registers.d = cpu.res(4, cpu.registers.d);
            2
        }
        0xA3 => {
            // res 4, E
            cpu.registers.e = cpu.res(4, cpu.registers.e);
            2
        }
        0xA4 => {
            // res 4, h
            cpu.registers.h = cpu.res(4, cpu.registers.h);
            2
        }
        0xA5 => {
            // res 4, l
            cpu.registers.l = cpu.res(4, cpu.registers.l);
            2
        }
        0xA6 => {
            // res 4, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.res(4, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0xA7 => {
            // res 4, A
            cpu.registers.a = cpu.res(4, cpu.registers.a);
            2
        }
        0xA8 => {
            // res 5, B
            cpu.registers.b = cpu.res(5, cpu.registers.b);
            2
        }
        0xA9 => {
            // res 5, C
            cpu.registers.c = cpu.res(5, cpu.registers.c);
            2
        }
        0xAA => {
            // res 5, D
            cpu.registers.d = cpu.res(5, cpu.registers.d);
            2
        }
        0xAB => {
            // res 5, E
            cpu.registers.e = cpu.res(5, cpu.registers.e);
            2
        }
        0xAC => {
            // res 5, h
            cpu.registers.h = cpu.res(5, cpu.registers.h);
            2
        }
        0xAD => {
            // res 5, l
            cpu.registers.l = cpu.res(5, cpu.registers.l);
            2
        }
        0xAE => {
            // res 5, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.res(5, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0xAF => {
            // res 5, A
            cpu.registers.a = cpu.res(5, cpu.registers.a);
            2
        }
        0xB0 => {
            // res 6, B
            cpu.registers.b = cpu.res(6, cpu.registers.b);
            2
        }
        0xB1 => {
            // res 6, C
            cpu.registers.c = cpu.res(6, cpu.registers.c);
            2
        }
        0xB2 => {
            // res 6, D
            cpu.registers.d = cpu.res(6, cpu.registers.d);
            2
        }
        0xB3 => {
            // res 6, E
            cpu.registers.e = cpu.res(6, cpu.registers.e);
            2
        }
        0xB4 => {
            // res 6, h
            cpu.registers.h = cpu.res(6, cpu.registers.h);
            2
        }
        0xB5 => {
            // res 6, l
            cpu.registers.l = cpu.res(6, cpu.registers.l);
            2
        }
        0xB6 => {
            // res 6, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.res(6, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0xB7 => {
            // res 6, A
            cpu.registers.a = cpu.res(6, cpu.registers.a);
            2
        }
        0xB8 => {
            // res 7, B
            cpu.registers.b = cpu.res(7, cpu.registers.b);
            2
        }
        0xB9 => {
            // res 7, C
            cpu.registers.c = cpu.res(7, cpu.registers.c);
            2
        }
        0xBA => {
            // res 7, D
            cpu.registers.d = cpu.res(7, cpu.registers.d);
            2
        }
        0xBB => {
            // res 7, E
            cpu.registers.e = cpu.res(7, cpu.registers.e);
            2
        }
        0xBC => {
            // res 7, h
            cpu.registers.h = cpu.res(7, cpu.registers.h);
            2
        }
        0xBD => {
            // res 7, l
            cpu.registers.l = cpu.res(7, cpu.registers.l);
            2
        }
        0xBE => {
            // res 7, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.res(7, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0xBF => {
            // res 7, A
            cpu.registers.a = cpu.res(7, cpu.registers.a);
            2
        }
        0xC0 => {
            // set 0, B
            cpu.registers.b = cpu.set(0, cpu.registers.b);
            2
        }
        0xC1 => {
            // set 0, C
            cpu.registers.c = cpu.set(0, cpu.registers.c);
            2
        }
        0xC2 => {
            // set 0, D
            cpu.registers.d = cpu.set(0, cpu.registers.d);
            2
        }
        0xC3 => {
            // set 0, E
            cpu.registers.e = cpu.set(0, cpu.registers.e);
            2
        }
        0xC4 => {
            // set 0, h
            cpu.registers.h = cpu.set(0, cpu.registers.h);
            2
        }
        0xC5 => {
            // set 0, l
            cpu.registers.l = cpu.set(0, cpu.registers.l);
            2
        }
        0xC6 => {
            // set 0, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.set(0, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0xC7 => {
            // set 0, A
            cpu.registers.a = cpu.set(0, cpu.registers.a);
            2
        }
        0xC8 => {
            // set 1, B
            cpu.registers.b = cpu.set(1, cpu.registers.b);
            2
        }
        0xC9 => {
            // set 1, C
            cpu.registers.c = cpu.set(1, cpu.registers.c);
            2
        }
        0xCA => {
            // set 1, D
            cpu.registers.d = cpu.set(1, cpu.registers.d);
            2
        }
        0xCB => {
            // set 1, E
            cpu.registers.e = cpu.set(1, cpu.registers.e);
            2
        }
        0xCC => {
            // set 1, h
            cpu.registers.h = cpu.set(1, cpu.registers.h);
            2
        }
        0xCD => {
            // set 1, l
            cpu.registers.l = cpu.set(1, cpu.registers.l);
            2
        }
        0xCE => {
            // set 1, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.set(1, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0xCF => {
            // set 1, A
            cpu.registers.a = cpu.set(1, cpu.registers.a);
            2
        }
        0xD0 => {
            // set 2, B
            cpu.registers.b = cpu.set(2, cpu.registers.b);
            2
        }
        0xD1 => {
            // set 2, C
            cpu.registers.c = cpu.set(2, cpu.registers.c);
            2
        }
        0xD2 => {
            // set 2, D
            cpu.registers.d = cpu.set(2, cpu.registers.d);
            2
        }
        0xD3 => {
            // set 2, E
            cpu.registers.e = cpu.set(2, cpu.registers.e);
            2
        }
        0xD4 => {
            // set 2, h
            cpu.registers.h = cpu.set(2, cpu.registers.h);
            2
        }
        0xD5 => {
            // set 2, l
            cpu.registers.l = cpu.set(2, cpu.registers.l);
            2
        }
        0xD6 => {
            // set 2, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.set(2, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0xD7 => {
            // set 2, A
            cpu.registers.a = cpu.set(2, cpu.registers.a);
            2
        }
        0xD8 => {
            // set 3, B
            cpu.registers.b = cpu.set(3, cpu.registers.b);
            2
        }
        0xD9 => {
            // set 3, C
            cpu.registers.c = cpu.set(3, cpu.registers.c);
            2
        }
        0xDA => {
            // set 3, D
            cpu.registers.d = cpu.set(3, cpu.registers.d);
            2
        }
        0xDB => {
            // set 3, E
            cpu.registers.e = cpu.set(3, cpu.registers.e);
            2
        }
        0xDC => {
            // set 3, h
            cpu.registers.h = cpu.set(3, cpu.registers.h);
            2
        }
        0xDD => {
            // set 3, l
            cpu.registers.l = cpu.set(3, cpu.registers.l);
            2
        }
        0xDE => {
            // set 3, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.set(3, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0xDF => {
            // set 3, A
            cpu.registers.a = cpu.set(3, cpu.registers.a);
            2
        }
        0xE0 => {
            // set 4, B
            cpu.registers.b = cpu.set(4, cpu.registers.b);
            2
        }
        0xE1 => {
            // set 4, C
            cpu.registers.c = cpu.set(4, cpu.registers.c);
            2
        }
        0xE2 => {
            // set 4, D
            cpu.registers.d = cpu.set(4, cpu.registers.d);
            2
        }
        0xE3 => {
            // set 4, E
            cpu.registers.e = cpu.set(4, cpu.registers.e);
            2
        }
        0xE4 => {
            // set 4, h
            cpu.registers.h = cpu.set(4, cpu.registers.h);
            2
        }
        0xE5 => {
            // set 4, l
            cpu.registers.l = cpu.set(4, cpu.registers.l);
            2
        }
        0xE6 => {
            // set 4, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.set(4, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0xE7 => {
            // set 4, A
            cpu.registers.a = cpu.set(4, cpu.registers.a);
            2
        }
        0xE8 => {
            // set 5, B
            cpu.registers.b = cpu.set(5, cpu.registers.b);
            2
        }
        0xE9 => {
            // set 5, C
            cpu.registers.c = cpu.set(5, cpu.registers.c);
            2
        }
        0xEA => {
            // set 5, D
            cpu.registers.d = cpu.set(5, cpu.registers.d);
            2
        }
        0xEB => {
            // set 5, E
            cpu.registers.e = cpu.set(5, cpu.registers.e);
            2
        }
        0xEC => {
            // set 5, h
            cpu.registers.h = cpu.set(5, cpu.registers.h);
            2
        }
        0xED => {
            // set 5, l
            cpu.registers.l = cpu.set(5, cpu.registers.l);
            2
        }
        0xEE => {
            // set 5, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.set(5, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0xEF => {
            // set 5, A
            cpu.registers.a = cpu.set(5, cpu.registers.a);
            2
        }
        0xF0 => {
            // set 6, B
            cpu.registers.b = cpu.set(6, cpu.registers.b);
            2
        }
        0xF1 => {
            // set 6, C
            cpu.registers.c = cpu.set(6, cpu.registers.c);
            2
        }
        0xF2 => {
            // set 6, D
            cpu.registers.d = cpu.set(6, cpu.registers.d);
            2
        }
        0xF3 => {
            // set 6, E
            cpu.registers.e = cpu.set(6, cpu.registers.e);
            2
        }
        0xF4 => {
            // set 6, h
            cpu.registers.h = cpu.set(6, cpu.registers.h);
            2
        }
        0xF5 => {
            // set 6, l
            cpu.registers.l = cpu.set(6, cpu.registers.l);
            2
        }
        0xF6 => {
            // set 6, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.set(6, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0xF7 => {
            // set 6, A
            cpu.registers.a = cpu.set(6, cpu.registers.a);
            2
        }
        0xF8 => {
            // set 7, B
            cpu.registers.b = cpu.set(7, cpu.registers.b);
            2
        }
        0xF9 => {
            // set 7, C
            cpu.registers.c = cpu.set(7, cpu.registers.c);
            2
        }
        0xFA => {
            // set 7, D
            cpu.registers.d = cpu.set(7, cpu.registers.d);
            2
        }
        0xFB => {
            // set 7, E
            cpu.registers.e = cpu.set(7, cpu.registers.e);
            2
        }
        0xFC => {
            // set 7, h
            cpu.registers.h = cpu.set(7, cpu.registers.h);
            2
        }
        0xFD => {
            // set 7, l
            cpu.registers.l = cpu.set(7, cpu.registers.l);
            2
        }
        0xFE => {
            // set 7, (HL)
            let value = mmu.read_byte(cpu.get_hl());
            let result = cpu.set(7, value);
            mmu.write_byte(cpu.get_hl(), result);
            4
        }
        0xFF => {
            // set 7, A
            cpu.registers.a = cpu.set(7, cpu.registers.a);
            2
        }
    }
}
