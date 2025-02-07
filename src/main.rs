use std::{iter::Sum, result};

mod opCodes;

const MEMORY_SIZE: usize = 65536;
const ROM_BANK_0: usize = 0x0000; // ROM Bank 0 (32KB) HOME BANK
const ROM_BANK_1: usize = 0x4000; // ROM Bank 1 (32KB)
const VRAM: usize = 0x8000; // VRAM (8KB) Background tiles
const CARTRIDGE_RAM: usize = 0xA000;
const WORK_RAM: usize = 0xC000; // RAM Bank 0 (8KB)
                                // Space not used
const OAM: usize = 0xFE00; // OAM (Sprites) (160 bytes) also tiles
                           //Space not used
const IO_REGISTERS: usize = 0xFF00; // IO Registros (80 bytes)
const HIGH_RAM: usize = 0xFF80; // Memoria de alto rendimiento (128 bytes) //Acceso un ciclo mas rapido

struct Registers {
    A: u8,
    F: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    H: u8,
    L: u8,
    PC: u16,
    SP: u16,
    IR: u8, // Instruction register
    IE: u8, // Interrupt enable
}
impl Registers {
    fn new() -> Registers {
        Registers {
            A: 0,
            F: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            H: 0,
            L: 0,
            PC: 0,
            SP: 0,
            IR: 0,
            IE: 0,
        }
    }
}

pub struct CPU {
    registers: Registers,
    memory: [u8; MEMORY_SIZE], // Memoria de la CPU

                               /*
                               FLAGS: Bits 7-4 de F

                               ZF:bool,    // Si es 0
                               NF:bool,    // Si es resta
                               HF:bool,    // Si hubo carry del bit 3 al 4
                               CF:bool,    // Si hay acarreo fuera de rango */
}

/*
LD es genérico ya que no tienes que justificar que el tipo T cumpla nada (sumar, restar, bitwise, etc)
Los que trabajan con u16 son los que pueden recibir de input u8 o u16. El trait Operand se encarga de hacer la conversión
Los que trabajan con <u8> es porque unicamente se aplican a A o un registro de 8 bits, no a uno cualquiera como los de arriba
*/
impl CPU {
    fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            memory: [0; MEMORY_SIZE], // Inicializa la memoria a ceros
        }
    }

    fn get_af(&self) -> u16 {
        ((self.registers.A as u16) << 8) | (self.registers.F as u16)
    }
    fn get_bc(&self) -> u16 {
        ((self.registers.B as u16) << 8) | (self.registers.C as u16)
    }
    fn get_de(&self) -> u16 {
        ((self.registers.D as u16) << 8) | (self.registers.E as u16)
    }
    fn get_hl(&self) -> u16 {
        ((self.registers.H as u16) << 8) | (self.registers.L as u16)
    }
    fn set_af(&mut self, value: u16) {
        self.registers.A = (value >> 8) as u8;
        self.registers.F = (value & 0xFF) as u8;
    }
    fn set_bc(&mut self, value: u16) {
        self.registers.B = (value >> 8) as u8;
        self.registers.C = (value & 0xFF) as u8;
    }
    fn set_de(&mut self, value: u16) {
        self.registers.D = (value >> 8) as u8;
        self.registers.E = (value & 0xFF) as u8;
    }
    fn set_hl(&mut self, value: u16) {
        self.registers.H = (value >> 8) as u8;
        self.registers.L = (value & 0xFF) as u8;
    }

    /*  fn LD<T>(&mut self, to: impl Operand<T>, from: impl Operand<T>) {
        let value = from.read(self);
        to.write(self, value);
    }

    fn POP(&mut self, To: RegisterPair) {
        let value = (self.memory[(self.SP + 1) as usize] as u16) << 8
            | self.memory[self.SP as usize] as u16;
        self.set_register_pair(To, value);
        self.SP += 2;
    }*/

    fn update_flags(&mut self, zero: bool, carry: bool, half_carry: bool, substract: bool) {
        self.F = 0;
        if zero {
            self.F |= 0b1000_0000;
        }
        if substract {
            self.F |= 0b0100_0000;
        }
        if half_carry {
            self.F |= 0b0010_0000;
        }
        if carry {
            self.F |= 0b0001_0000;
        }
    }

    fn ADD(&mut self, n1: u16, n2: u16) -> u16 {
        let (result, carry) = n1.overflowing_add(n2);
        self.update_flags(
            result == 0,
            carry,
            (n1 & 0x0FFF) + (n2 & 0x0FFF) > 0x0FFF,
            false,
        );
        result
    }

    fn SUB(&mut self, n1: u16, n2: u16) -> u16 {
        let (result, carry) = n1.overflowing_sub(n2);

        self.update_flags(result == 0, carry, (n1 & 0x0F) < (n2 & 0x0F), true);

        result
    }

    fn ADC(&mut self, n1: u16, n2: u16) -> u16 {
        let carry = if (self.F & 0b0001_0000) != 0 { 1 } else { 0 };
        let (result, carry2) = n1.overflowing_add(n2);

        self.update_flags(
            result == 0,
            carry2,
            (n1 & 0x0F) + (n2 & 0x0F) + carry > 0x0F,
            false,
        );

        result
    }

    fn SBC(&mut self, n1: u16, n2: u16) -> u16 {
        let carry = if (self.F & 0b0001_0000) != 0 { 1 } else { 0 };
        let (result, carry2) = n1.overflowing_sub(n2 + carry);

        self.update_flags(result == 0, carry2, (n1 & 0x0F) < (n2 & 0x0F) + carry, true);

        result
    }

    fn AND(&mut self, op: u8) {
        self.A &= op.read(self);
        self.update_flags(self.A == 0, false, true, false);
    }

    fn OR(&mut self, op: u8) {
        self.A |= op.read(self);
        self.update_flags(self.A == 0, false, false, false);
    }

    fn XOR(&mut self, op: u8) {
        self.A ^= op.read(self);
        self.update_flags(self.A == 0, false, false, false);
    }

    fn CP(&mut self, op: u8) {
        // Compara. Comprueba la resta pero no guarda el resultado
        let result = self.A as u16 - op.read(self) as u16;
        self.update_flags(
            result == 0,
            result > 0xFF,
            (self.A & 0x0F) + (op.read(self) & 0x0F) > 0x0F,
            false,
        );
    }

    fn INC(&mut self, op: u16) -> u16 {
        let value = op.read(self);
        let result = value.wrapping_add(1);
        self.update_flags(result as u8 == 0, false, (value & 0x0F) + 1 > 0x0F, false);

        result
    }

    fn DEC(&mut self, op: u16) -> u16 {
        let value = op.read(self);
        let result = value.wrapping_sub(1);
        self.update_flags(result as u8 == 0, false, (value & 0x0F) < 1, true);

        result
    }

    fn RLCA(&mut self) {
        // Mueve el bit 7 de A al bit 0 y al bit de carry
    }

    fn RLC<T>(&mut self, op: u8) {
        let value = op.read(self);
        let seven = value >> 7 & 1 != 0;

        let new_value = (value << 1) | (seven as u8);
        op.write(self, new_value);
        self.update_flags(value == 0, seven, false, false);
    }

    fn RL<T>(&mut self, op: u8) {
        let value = op.read(self);
        let seven = value >> 7 & 1 != 0;
        let carry = (self.F & 0b0001_0000) >> 4;

        let new_value = (value << 1) | carry;
        op.write(self, new_value);
        self.update_flags(value == 0, seven, false, false);
    }

    fn RLA(&mut self) {
        // Mueve el bit 7 de a al carry y el carry al 0
        self.RL::<u8>(Register8::A);
    }

    fn fetch_byte(&mut self) -> u8 {
        let op = self.memory[self.PC as usize];
        self.PC += 1;
        op
    }

    fn fetch_word(&mut self) -> u16 {
        let op = (self.memory[self.PC as usize] as u16)
            | ((self.memory[(self.PC + 1) as usize] as u16) << 8);
        self.PC += 2;
        op
    }
}

fn main() {
    let mut cpu = CPU::new();
}
