use crate::mmu::MMU;
use crate::op_codes::execute_opcode;

const DIV_INCREMENT_RATE: u32 = 256 / 4; // M-cycles

pub struct Registers {
    pub A: u8,
    pub F: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub H: u8,
    pub L: u8,
    pub PC: u16,
    pub SP: u16,
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
            A: 0x01,
            F: 0xB0,
            B: 0x00,
            C: 0x13,
            D: 0x00,
            E: 0xD8,
            H: 0x01,
            L: 0x4D,
            PC: 0x0100, // Start address of the program
            SP: 0xFFFE, // Initial stack pointer

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
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            mmu: MMU::new(),
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
        ((self.registers.A as u16) << 8) | (self.registers.F as u16)
    }
    pub fn get_bc(&self) -> u16 {
        ((self.registers.B as u16) << 8) | (self.registers.C as u16)
    }
    pub fn get_de(&self) -> u16 {
        ((self.registers.D as u16) << 8) | (self.registers.E as u16)
    }
    pub fn get_hl(&self) -> u16 {
        ((self.registers.H as u16) << 8) | (self.registers.L as u16)
    }
    pub fn set_af(&mut self, value: u16) {
        self.registers.A = (value >> 8) as u8;
        self.registers.F = (value & 0xFF) as u8;
    }
    pub fn set_bc(&mut self, value: u16) {
        self.registers.B = (value >> 8) as u8;
        self.registers.C = (value & 0xFF) as u8;
    }
    pub fn set_de(&mut self, value: u16) {
        self.registers.D = (value >> 8) as u8;
        self.registers.E = (value & 0xFF) as u8;
    }
    pub fn set_hl(&mut self, value: u16) {
        self.registers.H = (value >> 8) as u8;
        self.registers.L = (value & 0xFF) as u8;
    }
    pub fn set_ZF(&mut self, value: bool) {
        self.registers.F = self.registers.F & 0b0111_1111 | (value as u8) << 7;
    }
    pub fn set_NF(&mut self, value: bool) {
        self.registers.F = self.registers.F & 0b1011_1111 | (value as u8) << 6;
    }
    pub fn set_HF(&mut self, value: bool) {
        self.registers.F = self.registers.F & 0b1101_1111 | (value as u8) << 5;
    }
    pub fn set_CF(&mut self, value: bool) {
        self.registers.F = self.registers.F & 0b1110_1111 | (value as u8) << 4;
    }

    pub fn get_ZF(&self) -> bool {
        (self.registers.F & 0b1000_0000) != 0
    }
    pub fn get_NF(&self) -> bool {
        (self.registers.F & 0b0100_0000) != 0
    }
    pub fn get_HF(&self) -> bool {
        (self.registers.F & 0b0010_0000) != 0
    }
    pub fn get_CF(&self) -> bool {
        (self.registers.F & 0b0001_0000) != 0
    }

    pub fn update_flags(&mut self, zero: bool, carry: bool, half_carry: bool, substract: bool) {
        self.registers.F = 0;
        if zero {
            self.registers.F |= 0b1000_0000;
        }
        if substract {
            self.registers.F |= 0b0100_0000;
        }
        if half_carry {
            self.registers.F |= 0b0010_0000;
        }
        if carry {
            self.registers.F |= 0b0001_0000;
        }
    }

    pub fn ADD16(&mut self, n1: u16, n2: u16) -> u16 {
        let (result, carry) = n1.overflowing_add(n2);
        self.update_flags(
            self.get_ZF(),
            carry,
            (n1 & 0x0FFF) + (n2 & 0x0FFF) > 0x0FFF,
            false,
        );
        result
    }

    pub fn ADD8(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.A;
        let (result, carry) = n1.overflowing_add(n2);
        self.update_flags(result == 0, carry, (n1 & 0x0F) + (n2 & 0x0F) > 0x0F, false);
        result
    }

    pub fn SUB(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.A;
        let (result, carry) = n1.overflowing_sub(n2);

        self.update_flags(result == 0, carry, (n1 & 0x0F) < (n2 & 0x0F), true);

        result
    }

    pub fn ADC(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.A;
        let carry_prev = self.get_CF() as u8;
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

    pub fn SBC(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.A;
        let carry_prev = self.get_CF() as u8;
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

    pub fn AND(&mut self, num: u8) {
        self.registers.A &= num;
        self.update_flags(self.registers.A == 0, false, true, false);
    }

    pub fn OR(&mut self, num: u8) {
        self.registers.A |= num;
        self.update_flags(self.registers.A == 0, false, false, false);
    }

    pub fn XOR(&mut self, num: u8) {
        self.registers.A ^= num;
        self.update_flags(self.registers.A == 0, false, false, false);
    }

    pub fn CP(&mut self, num: u8) {
        // Compara. Comprueba la resta pero no guarda el resultado
        let (result, carry) = self.registers.A.overflowing_sub(num);
        self.update_flags(
            result == 0,
            carry,
            (self.registers.A & 0xF) < (num & 0xF),
            true,
        );
    }

    pub fn INC(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.update_flags(
            result as u8 == 0,
            self.get_CF(),
            (value & 0x0F) + 1 > 0x0F,
            false,
        );

        result
    }

    pub fn DEC(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.update_flags(result as u8 == 0, self.get_CF(), (value & 0x0F) < 1, true);

        result
    }

    pub fn RLC(&mut self, value: u8) -> u8 {
        let seven = value >> 7 & 1 != 0;
        let result = (value << 1) | (seven as u8);
        self.update_flags(result == 0, seven, false, false);
        result
    }

    pub fn RLCA(&mut self) {
        self.registers.A = self.RLC(self.registers.A);
        self.set_ZF(false);
    }

    pub fn RL(&mut self, value: u8) -> u8 {
        let seven = value >> 7 & 1 != 0;
        let carry = (self.registers.F & 0b0001_0000) >> 4;
        let result = (value << 1) | carry;
        self.update_flags(result == 0, seven, false, false);
        result
    }

    pub fn RLA(&mut self) {
        self.registers.A = self.RL(self.registers.A);
        self.set_ZF(false);
    }

    pub fn RRC(&mut self, value: u8) -> u8 {
        let bit = 0b0000_0001 & value;
        let result = (value >> 1) | (bit << 7);
        self.update_flags(result == 0, bit != 0, false, false);
        result
    }

    pub fn RRCA(&mut self) {
        self.registers.A = self.RRC(self.registers.A);
        self.set_ZF(false);
    }

    pub fn RR(&mut self, value: u8) -> u8 {
        let bit = 0b0000_0001 & value;
        let carry = if self.get_CF() { 1 } else { 0 };
        let result = (value >> 1) | (carry << 7);
        self.update_flags(result == 0, bit != 0, false, false);

        result
    }

    pub fn RRA(&mut self) {
        self.registers.A = self.RR(self.registers.A);
        self.set_ZF(false);
    }

    pub fn SLA(&mut self, value: u8) -> u8 {
        //TODO
        let seven = value >> 7 & 1 != 0;
        let result = value << 1;
        self.update_flags(result == 0, seven, false, false);
        result
    }

    pub fn SRA(&mut self, value: u8) -> u8 {
        let seven = value & 0b1000_0000;
        let bit = value & 0b0000_0001;
        let result = (value >> 1) | seven;
        self.update_flags(result == 0, bit != 0, false, false);
        result
    }

    pub fn SRL(&mut self, value: u8) -> u8 {
        let bit = value & 0b0000_0001;
        let result = value >> 1;
        self.update_flags(result == 0, bit != 0, false, false);
        result
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let op = self.mmu.read_byte(self.registers.PC);
        self.registers.PC += 1;
        op
    }

    pub fn fetch_word(&mut self) -> u16 {
        let op = self.mmu.read_word(self.registers.PC);
        self.registers.PC += 2;
        op
    }

    pub fn JR(&mut self, condition: bool) {
        let offset: i8 = self.fetch_byte() as i8;
        if condition {
            self.registers.PC = (self.registers.PC as i32 + offset as i32) as u16;
        }
    }

    pub fn JP(&mut self, condition: bool) {
        let address = self.fetch_word();
        if condition {
            self.registers.PC = address;
        }
    }

    pub fn RES(&mut self, bit: u8, num: u8) -> u8 {
        num & !(1 << bit)
    }

    pub fn SET(&mut self, bit: u8, num: u8) -> u8 {
        num | (1 << bit)
    }

    pub fn BIT(&mut self, bit: u8, num: u8) {
        let res = (num & (1 << bit)) == 0;

        self.update_flags(res, self.get_CF(), true, false);
    }

    pub fn DAA(&mut self) {
        let mut a = self.registers.A;
        let mut adjust: u8 = 0;
        if self.get_HF() || (!self.get_NF() && (a & 0xF) > 9) {
            adjust |= 0x06;
        }
        if self.get_CF() || (!self.get_NF() && a > 0x99) {
            adjust |= 0x60;
            self.set_CF(true);
        }
        if self.get_NF() {
            a = a.wrapping_sub(adjust);
        } else {
            a = a.wrapping_add(adjust);
        }
        self.set_ZF(a == 0);
        self.set_HF(false);
        self.registers.A = a;
    }

    pub fn CPL(&mut self) {
        self.registers.A = !self.registers.A;

        self.set_NF(true);
        self.set_HF(true);
    }

    pub fn SWAP(&mut self, value: u8) -> u8 {
        let low = value & 0x0F;
        let high = value & 0xF0;
        let res = (low << 4) | (high >> 4);
        self.update_flags(res == 0, false, false, false);
        res
    }

    pub fn CCF(&mut self) {
        self.set_CF(!self.get_CF());
        self.set_NF(false);
        self.set_HF(false);
    }

    pub fn SCF(&mut self) {
        self.set_CF(true);
        self.set_NF(false);
        self.set_HF(false);
    }

    pub fn POP(&mut self) -> u16 {
        let value = self.mmu.read_word(self.registers.SP);
        self.registers.SP = self.registers.SP.wrapping_add(2);
        value
    }

    pub fn PUSH(&mut self, value: u16) {
        self.registers.SP -= 2;
        self.mmu.write_word(self.registers.SP, value);
    }

    pub fn RST(&mut self, address: u16) {
        self.PUSH(self.registers.PC);
        self.registers.PC = address;
    }

    pub fn RET(&mut self, condition: bool) {
        if condition {
            self.registers.PC = self.POP();
        }
    }

    pub fn CALL(&mut self, condition: bool) {
        let address = self.fetch_word();

        if condition {
            self.PUSH(self.registers.PC);
            self.registers.PC = address;
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
                self.PUSH(self.registers.PC); // Push the current program counter onto the stack
                self.registers.PC = 0x40; // Jump to the interrupt handler
                return 5;
            } else if self.get_if(InterruptCode::Lcd) && self.get_ie(InterruptCode::Lcd) {
                self.ime = false;
                self.set_if(InterruptCode::Lcd, false);
                //cpu.nop() x2
                self.PUSH(self.registers.PC);
                self.registers.PC = 0x48;
                return 5;
            } else if self.get_if(InterruptCode::Timer) && self.get_ie(InterruptCode::Timer) {
                self.ime = false;
                self.set_if(InterruptCode::Timer, false);
                //cpu.nop() x2
                self.PUSH(self.registers.PC);
                self.registers.PC = 0x50;
                return 5;
            } else if self.get_if(InterruptCode::Serial) && self.get_ie(InterruptCode::Serial) {
                self.ime = false;
                self.set_if(InterruptCode::Serial, false);
                //cpu.nop() x2
                self.PUSH(self.registers.PC);
                self.registers.PC = 0x58;
                return 5;
            } else if self.get_if(InterruptCode::Joypad) && self.get_ie(InterruptCode::Joypad) {
                self.ime = false;
                self.set_if(InterruptCode::Joypad, false);
                //cpu.nop() x2
                self.PUSH(self.registers.PC);
                self.registers.PC = 0x60;
                return 5;
            }
        }
        return 0;
    }
}
