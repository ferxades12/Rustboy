use crate::mmu::MMU;
use crate::op_codes::execute_opcode;

const DIV_INCREMENT_RATE: u32 = 256 / 4; // M-cycles

pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

pub enum InterruptCode {
    Vblank = 0,
    Lcd = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}

pub enum ControlRegisters {
    IE = 0xFFFF, // Interrupt Enable // 7 6 5 Joypad Serial Timer LCD V-Blank
    IF = 0xFF0F, // Interrupt Flag (Requests an interrupt) // 7 6 5 Joypad Serial Timer LCD V-Blank
    DIV = 0xFF04,
    TIMA = 0xFF05,
    TMA = 0xFF06,
    TAC = 0xFF07,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100, // Start address of the program
            sp: 0xFFFE, // Initial stack pointer

                        /*
                        FLAGS: Bits 7-4 de F

                        ZF:bool,    // Si es 0
                        NF:bool,    // Si es resta
                        HF:bool,    // Si hubo carry del bit 3 al 4
                        CF:bool,    // Si hay acarreo fuera de rango */
        }
    }
}

pub struct CPU {
    pub registers: Registers,
    pub mmu: MMU,
    pub ei_flag: bool,   // Flag de interrupciones
    pub stop_flag: bool, // Flag de parada
    pub halt_flag: bool,
    pub div_counter: u32,
    pub tima_counter: u32,
    pub ime: bool, // Interrupciones maestras habilitadas
}

impl CPU {
    pub fn new(mmu: MMU) -> CPU {
        CPU {
            registers: Registers::new(),
            mmu: mmu,
            ei_flag: false,
            stop_flag: false,
            halt_flag: false,
            div_counter: 0,
            tima_counter: 0,
            ime: false,
        }
    }

    pub fn get_tac_frequency(&self) -> u32 {
        match self.mmu.read_byte(ControlRegisters::TAC as u16) & 0b11 {
            0b00 => 256, // M-cycles
            0b01 => 4,
            0b10 => 16,
            0b11 => 64,
            _ => {
                panic!("Invalid TAC frecuency");
            }
        }
    }
    pub fn get_ime(&self) -> bool {
        self.ime
    }

    pub fn set_ime(&mut self, value: bool) {
        self.ime = value;
    }

    pub fn get_tac_enabled(&self) -> bool {
        (self.mmu.read_byte(ControlRegisters::TAC as u16) & 0b100) != 0
    }

    pub fn get_ie(&self, code: InterruptCode) -> bool {
        (self.mmu.read_byte(ControlRegisters::IE as u16) & (1 << code as u8)) != 0
    }

    pub fn get_if(&self, code: InterruptCode) -> bool {
        (self.mmu.read_byte(ControlRegisters::IF as u16) & (1 << code as u8)) != 0
    }

    pub fn set_ie(&mut self, code: InterruptCode, value: bool) {
        if value {
            self.mmu.write_byte(
                ControlRegisters::IE as u16,
                self.mmu.read_byte(ControlRegisters::IE as u16) | (1 << code as u8),
            );
        } else {
            self.mmu.write_byte(
                ControlRegisters::IE as u16,
                self.mmu.read_byte(ControlRegisters::IE as u16) & !(1 << code as u8),
            );
        }
    }

    pub fn set_if(&mut self, code: InterruptCode, value: bool) {
        if value {
            self.mmu.write_byte(
                ControlRegisters::IF as u16,
                self.mmu.read_byte(ControlRegisters::IF as u16) | (1 << code as u8),
            );
        } else {
            self.mmu.write_byte(
                ControlRegisters::IF as u16,
                self.mmu.read_byte(ControlRegisters::IF as u16) & !(1 << code as u8),
            );
        }
    }

    pub fn get_af(&self) -> u16 {
        ((self.registers.a as u16) << 8) | (self.registers.f as u16)
    }
    pub fn get_bc(&self) -> u16 {
        ((self.registers.b as u16) << 8) | (self.registers.c as u16)
    }
    pub fn get_de(&self) -> u16 {
        ((self.registers.d as u16) << 8) | (self.registers.e as u16)
    }
    pub fn get_hl(&self) -> u16 {
        ((self.registers.h as u16) << 8) | (self.registers.l as u16)
    }
    pub fn set_af(&mut self, value: u16) {
        self.registers.a = (value >> 8) as u8;
        self.registers.f = (value & 0xFF) as u8;
    }
    pub fn set_bc(&mut self, value: u16) {
        self.registers.b = (value >> 8) as u8;
        self.registers.c = (value & 0xFF) as u8;
    }
    pub fn set_de(&mut self, value: u16) {
        self.registers.d = (value >> 8) as u8;
        self.registers.e = (value & 0xFF) as u8;
    }
    pub fn set_hl(&mut self, value: u16) {
        self.registers.h = (value >> 8) as u8;
        self.registers.l = (value & 0xFF) as u8;
    }
    pub fn set_zf(&mut self, value: bool) {
        self.registers.f = self.registers.f & 0b0111_1111 | (value as u8) << 7;
    }
    pub fn set_nf(&mut self, value: bool) {
        self.registers.f = self.registers.f & 0b1011_1111 | (value as u8) << 6;
    }
    pub fn set_hf(&mut self, value: bool) {
        self.registers.f = self.registers.f & 0b1101_1111 | (value as u8) << 5;
    }
    pub fn set_cf(&mut self, value: bool) {
        self.registers.f = self.registers.f & 0b1110_1111 | (value as u8) << 4;
    }

    pub fn get_zf(&self) -> bool {
        (self.registers.f & 0b1000_0000) != 0
    }
    pub fn get_nf(&self) -> bool {
        (self.registers.f & 0b0100_0000) != 0
    }
    pub fn get_hf(&self) -> bool {
        (self.registers.f & 0b0010_0000) != 0
    }
    pub fn get_cf(&self) -> bool {
        (self.registers.f & 0b0001_0000) != 0
    }

    pub fn update_flags(&mut self, zero: bool, carry: bool, half_carry: bool, substract: bool) {
        self.registers.f = 0;
        if zero {
            self.registers.f |= 0b1000_0000;
        }
        if substract {
            self.registers.f |= 0b0100_0000;
        }
        if half_carry {
            self.registers.f |= 0b0010_0000;
        }
        if carry {
            self.registers.f |= 0b0001_0000;
        }
    }

    pub fn add16(&mut self, n1: u16, n2: u16) -> u16 {
        let (result, carry) = n1.overflowing_add(n2);
        self.update_flags(
            self.get_zf(),
            carry,
            (n1 & 0x0FFF) + (n2 & 0x0FFF) > 0x0FFF,
            false,
        );
        result
    }

    pub fn add8(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.a;
        let (result, carry) = n1.overflowing_add(n2);
        self.update_flags(result == 0, carry, (n1 & 0x0F) + (n2 & 0x0F) > 0x0F, false);
        result
    }

    pub fn sub(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.a;
        let (result, carry) = n1.overflowing_sub(n2);

        self.update_flags(result == 0, carry, (n1 & 0x0F) < (n2 & 0x0F), true);

        result
    }

    pub fn adc(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.a;
        let carry_prev = self.get_cf() as u8;
        let (result, carry1) = n1.overflowing_add(n2);
        let (result, carry2) = result.overflowing_add(carry_prev);

        self.update_flags(
            result == 0,
            carry1 | carry2,
            (n1 & 0x0F) + (n2 & 0x0F) + carry_prev > 0x0F,
            false,
        );

        result
    }

    pub fn sbc(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.a;
        let carry_prev = self.get_cf() as u8;
        let (result, carry1) = n1.overflowing_sub(n2);
        let (result, carry2) = result.overflowing_sub(carry_prev);

        self.update_flags(
            result == 0,
            carry1 | carry2,
            (n1 & 0x0F) < ((n2 & 0x0F) + carry_prev),
            true,
        );

        result
    }

    pub fn and(&mut self, num: u8) {
        self.registers.a &= num;
        self.update_flags(self.registers.a == 0, false, true, false);
    }

    pub fn or(&mut self, num: u8) {
        self.registers.a |= num;
        self.update_flags(self.registers.a == 0, false, false, false);
    }

    pub fn xor(&mut self, num: u8) {
        self.registers.a ^= num;
        self.update_flags(self.registers.a == 0, false, false, false);
    }

    pub fn cp(&mut self, num: u8) {
        // Compara. Comprueba la resta pero no guarda el resultado
        let (result, carry) = self.registers.a.overflowing_sub(num);
        self.update_flags(
            result == 0,
            carry,
            (self.registers.a & 0xF) < (num & 0xF),
            true,
        );
    }

    pub fn inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.update_flags(
            result as u8 == 0,
            self.get_cf(),
            (value & 0x0F) + 1 > 0x0F,
            false,
        );

        result
    }

    pub fn dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.update_flags(result as u8 == 0, self.get_cf(), (value & 0x0F) < 1, true);

        result
    }

    pub fn rlc(&mut self, value: u8) -> u8 {
        let seven = value >> 7 & 1 != 0;
        let result = (value << 1) | (seven as u8);
        self.update_flags(result == 0, seven, false, false);
        result
    }

    pub fn rlca(&mut self) {
        self.registers.a = self.rlc(self.registers.a);
        self.set_zf(false);
    }

    pub fn rl(&mut self, value: u8) -> u8 {
        let seven = value >> 7 & 1 != 0;
        let carry = (self.registers.f & 0b0001_0000) >> 4;
        let result = (value << 1) | carry;
        self.update_flags(result == 0, seven, false, false);
        result
    }

    pub fn rla(&mut self) {
        self.registers.a = self.rl(self.registers.a);
        self.set_zf(false);
    }

    pub fn rrc(&mut self, value: u8) -> u8 {
        let bit = 0b0000_0001 & value;
        let result = (value >> 1) | (bit << 7);
        self.update_flags(result == 0, bit != 0, false, false);
        result
    }

    pub fn rrca(&mut self) {
        self.registers.a = self.rrc(self.registers.a);
        self.set_zf(false);
    }

    pub fn rr(&mut self, value: u8) -> u8 {
        let bit = 0b0000_0001 & value;
        let carry = if self.get_cf() { 1 } else { 0 };
        let result = (value >> 1) | (carry << 7);
        self.update_flags(result == 0, bit != 0, false, false);

        result
    }

    pub fn rra(&mut self) {
        self.registers.a = self.rr(self.registers.a);
        self.set_zf(false);
    }

    pub fn sla(&mut self, value: u8) -> u8 {
        //TODO
        let seven = value >> 7 & 1 != 0;
        let result = value << 1;
        self.update_flags(result == 0, seven, false, false);
        result
    }

    pub fn sra(&mut self, value: u8) -> u8 {
        let seven = value & 0b1000_0000;
        let bit = value & 0b0000_0001;
        let result = (value >> 1) | seven;
        self.update_flags(result == 0, bit != 0, false, false);
        result
    }

    pub fn srl(&mut self, value: u8) -> u8 {
        let bit = value & 0b0000_0001;
        let result = value >> 1;
        self.update_flags(result == 0, bit != 0, false, false);
        result
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let op = self.mmu.read_byte(self.registers.pc);
        self.registers.pc += 1;
        op
    }

    pub fn fetch_word(&mut self) -> u16 {
        let op = self.mmu.read_word(self.registers.pc);
        self.registers.pc += 2;
        op
    }

    pub fn jr(&mut self, condition: bool) {
        let offset: i8 = self.fetch_byte() as i8;
        if condition {
            self.registers.pc = (self.registers.pc as i32 + offset as i32) as u16;
        }
    }

    pub fn jp(&mut self, condition: bool) {
        let address = self.fetch_word();
        if condition {
            self.registers.pc = address;
        }
    }

    pub fn res(&mut self, bit: u8, num: u8) -> u8 {
        num & !(1 << bit)
    }

    pub fn set(&mut self, bit: u8, num: u8) -> u8 {
        num | (1 << bit)
    }

    pub fn bit(&mut self, bit: u8, num: u8) {
        let res = (num & (1 << bit)) == 0;

        self.update_flags(res, self.get_cf(), true, false);
    }

    pub fn daa(&mut self) {
        let mut a = self.registers.a;
        let mut adjust: u8 = 0;
        if self.get_hf() || (!self.get_nf() && (a & 0xF) > 9) {
            adjust |= 0x06;
        }
        if self.get_cf() || (!self.get_nf() && a > 0x99) {
            adjust |= 0x60;
            self.set_cf(true);
        }
        if self.get_nf() {
            a = a.wrapping_sub(adjust);
        } else {
            a = a.wrapping_add(adjust);
        }
        self.set_zf(a == 0);
        self.set_hf(false);
        self.registers.a = a;
    }

    pub fn cpl(&mut self) {
        self.registers.a = !self.registers.a;

        self.set_nf(true);
        self.set_hf(true);
    }

    pub fn swap(&mut self, value: u8) -> u8 {
        let low = value & 0x0F;
        let high = value & 0xF0;
        let res = (low << 4) | (high >> 4);
        self.update_flags(res == 0, false, false, false);
        res
    }

    pub fn ccf(&mut self) {
        self.set_cf(!self.get_cf());
        self.set_nf(false);
        self.set_hf(false);
    }

    pub fn scf(&mut self) {
        self.set_cf(true);
        self.set_nf(false);
        self.set_hf(false);
    }

    pub fn pop(&mut self) -> u16 {
        let value = self.mmu.read_word(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        value
    }

    pub fn push(&mut self, value: u16) {
        self.registers.sp -= 2;
        self.mmu.write_word(self.registers.sp, value);
    }

    pub fn rst(&mut self, address: u16) {
        self.push(self.registers.pc);
        self.registers.pc = address;
    }

    pub fn ret(&mut self, condition: bool) {
        if condition {
            self.registers.pc = self.pop();
        }
    }

    pub fn call(&mut self, condition: bool) {
        let address = self.fetch_word();

        if condition {
            self.push(self.registers.pc);
            self.registers.pc = address;
        }
    }

    pub fn increment_div_register(&mut self) {
        if self.div_counter >= DIV_INCREMENT_RATE {
            self.mmu.write_byte(
                ControlRegisters::DIV as u16,
                self.mmu
                    .read_byte(ControlRegisters::DIV as u16)
                    .wrapping_add(1),
            );
            self.div_counter -= DIV_INCREMENT_RATE; // Reset the cycle counter
        }
    }

    pub fn increment_tima_register(&mut self) {
        if self.tima_counter >= self.get_tac_frequency() && self.get_tac_enabled() {
            // Checks overflow in TIMA and enable bit in TAC
            self.tima_counter -= self.get_tac_frequency(); // Reset the TIMA counter
            let (result, overflow) = self
                .mmu
                .read_byte(ControlRegisters::TIMA as u16)
                .overflowing_add(1);

            if overflow {
                // Requesti interrupt and reset TIMA
                self.set_if(InterruptCode::Timer, true);
                self.mmu.write_byte(
                    ControlRegisters::TIMA as u16,
                    self.mmu.read_byte(ControlRegisters::TMA as u16),
                );
            } else {
                // Increment TIMA
                self.mmu.write_byte(ControlRegisters::TIMA as u16, result);
            }
        }
    }

    pub fn step(&mut self) -> u32 {
        let mut cycles = 0;

        // Handle HALT
        if self.halt_flag {
            loop {
                // Still running div and tima registers
                self.increment_div_register();
                self.increment_tima_register();

                // Exit on interrupt
                if (self.mmu.read_byte(ControlRegisters::IF as u16)
                    & self.mmu.read_byte(ControlRegisters::IE as u16))
                    != 0
                {
                    self.halt_flag = false;
                    self.handle_interrupts();
                    break;
                }
            }
        } else {
            cycles += execute_opcode(self) as u32;
        }

        self.tima_counter = self.tima_counter.wrapping_add(cycles);
        self.div_counter = self.div_counter.wrapping_add(cycles);

        // Increment the DIV register
        self.increment_div_register();

        // Increment the TIMA register
        self.increment_tima_register();

        // Handle interrupts
        let cycles2 = self.handle_interrupts();

        self.tima_counter = self.tima_counter.wrapping_add(cycles2);
        self.div_counter = self.div_counter.wrapping_add(cycles2);

        // Increment the DIV register
        self.increment_div_register();

        // Increment the TIMA register
        self.increment_tima_register();

        cycles + cycles2
    }

    fn handle_interrupts(&mut self) -> u32 {
        if self.ei_flag {
            self.ei_flag = false;
            self.ime = true;
        }

        if self.ime {
            if self.get_if(InterruptCode::Vblank) && self.get_ie(InterruptCode::Vblank) {
                // Check both IME and IF
                self.ime = false;
                self.set_if(InterruptCode::Vblank, false); // Unset IME and IF
                self.push(self.registers.pc); // Push the current program counter onto the stack
                self.registers.pc = 0x40; // Jump to the interrupt handler
                return 5;
            } else if self.get_if(InterruptCode::Lcd) && self.get_ie(InterruptCode::Lcd) {
                self.ime = false;
                self.set_if(InterruptCode::Lcd, false);
                //cpu.nop() x2
                self.push(self.registers.pc);
                self.registers.pc = 0x48;
                return 5;
            } else if self.get_if(InterruptCode::Timer) && self.get_ie(InterruptCode::Timer) {
                self.ime = false;
                self.set_if(InterruptCode::Timer, false);
                //cpu.nop() x2
                self.push(self.registers.pc);
                self.registers.pc = 0x50;
                return 5;
            } else if self.get_if(InterruptCode::Serial) && self.get_ie(InterruptCode::Serial) {
                self.ime = false;
                self.set_if(InterruptCode::Serial, false);
                //cpu.nop() x2
                self.push(self.registers.pc);
                self.registers.pc = 0x58;
                return 5;
            } else if self.get_if(InterruptCode::Joypad) && self.get_ie(InterruptCode::Joypad) {
                self.ime = false;
                self.set_if(InterruptCode::Joypad, false);
                //cpu.nop() x2
                self.push(self.registers.pc);
                self.registers.pc = 0x60;
                return 5;
            }
        }
        return 0;
    }
}
