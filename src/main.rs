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
    fn set_ZF(&mut self, value: bool) {
        self.registers.F = self.registers.F & 0b0111_1111 | (value as u8) << 7;
    }
    fn set_NF(&mut self, value: bool) {
        self.registers.F = self.registers.F & 0b1011_1111 | (value as u8) << 6;
    }
    fn set_HF(&mut self, value: bool) {
        self.registers.F = self.registers.F & 0b1101_1111 | (value as u8) << 5;
    }
    fn set_CF(&mut self, value: bool) {
        self.registers.F = self.registers.F & 0b1110_1111 | (value as u8) << 4;
    }

    fn get_ZF(&self) -> bool {
        (self.registers.F & 0b1000_0000) != 0
    }
    fn get_NF(&self) -> bool {
        (self.registers.F & 0b0100_0000) != 0
    }
    fn get_HF(&self) -> bool {
        (self.registers.F & 0b0010_0000) != 0
    }
    fn get_CF(&self) -> bool {
        (self.registers.F & 0b0001_0000) != 0
    }


    fn update_flags(&mut self, zero: bool, carry: bool, half_carry: bool, substract: bool) {
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

    fn ADD16(&mut self, n1: u16, n2: u16) -> u16 {
        let (result, carry) = n1.overflowing_add(n2);
        self.update_flags(
            self.get_ZF(),
            carry,
            (n1 & 0x0FFF) + (n2 & 0x0FFF) > 0x0FFF,
            false,
        );
        result
    }

    fn ADD8(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.A;
        let (result, carry) = n1.overflowing_add(n2);
        self.update_flags(result == 0, carry, (n1 & 0x0F) + (n2 & 0x0F) > 0x0F, false);
        result
    }

    fn SUB(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.A;
        let (result, carry) = n1.overflowing_sub(n2);

        self.update_flags(result == 0, carry, (n1 & 0x0F) < (n2 & 0x0F), true);

        result
    }

    fn ADC(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.A;
        let carry_prev = self.get_CF() as u8;
        let (result, carry1) = n1.overflowing_add(n2);
        let (result, carry2) = result.overflowing_add(carry_prev);

        self.update_flags(result == 0, carry1 | carry2, (n1 & 0x0F) + (n2 & 0x0F) + carry_prev > 0x0F, false);

        result
    }

    fn SBC(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.A;
        let carry_prev = self.get_CF() as u8;
        let (result, carry1) = n1.overflowing_sub(n2);
        let (result, carry2) = result.overflowing_sub(carry_prev);

        self.update_flags(result == 0, carry1 | carry2, ((n1 & 0x0F) < ((n2 & 0x0F) + carry_prev)), true);

        result
    }

    fn AND(&mut self, num: u8) {
        self.registers.A &= num;
        self.update_flags(self.registers.A == 0, false, true, false);
    }

    fn OR(&mut self, num: u8) {
        self.registers.A |= num;
        self.update_flags(self.registers.A == 0, false, false, false);
    }

    fn XOR(&mut self, num: u8) {
        self.registers.A ^= num;
        self.update_flags(self.registers.A == 0, false, false, false);
    }

    fn CP(&mut self, num: u8) { // Compara. Comprueba la resta pero no guarda el resultado
        let (result, carry) = self.registers.A.overflowing_sub(num);
        self.update_flags(
            result == 0,
            carry,
            (self.registers.A & 0x0F) + (num & 0x0F) > 0x0F,
            true, 
        );
    }

    fn INC(&mut self, value:u8) -> u8 {
        let result = value.wrapping_add(1);
        self.update_flags(result as u8 == 0, self.get_CF(), (value & 0x0F) + 1 > 0x0F, false);

        result
    }

    fn DEC(&mut self, value:u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.update_flags(result as u8 == 0, self.get_CF(), (value & 0x0F) < 1, true);

        result
    }

    fn RLC(&mut self, value:u8) -> u8 { 
        let seven = value >> 7 & 1 != 0;
        let result = (value << 1) | (seven as u8);
        self.update_flags(result == 0, seven, false, false);
        result
    }

    fn RLCA(&mut self) {
        self.registers.A = self.RLC(self.registers.A);
        self.set_ZF(false);
    }

    fn RL(&mut self, value: u8) -> u8 {
        let seven = value >> 7 & 1 != 0;
        let carry = (self.registers.F & 0b0001_0000) >> 4;
        let result = (value << 1) | carry;
        self.update_flags(result == 0, seven, false, false);
        result
    }

    fn RLA(&mut self) {
        self.registers.A = self.RL(self.registers.A);
        self.set_ZF(false);
    }

    fn RRC(&mut self, value:u8) -> u8{
        let bit = 0b0000_0001 & value;
        let result = (value >> 1) | (bit << 7);
        self.update_flags(result == 0, bit != 0, false, false);
        result
    }
    
    fn RRCA(&mut self) {
        self.registers.A = self.RRC(self.registers.A);
        self.set_ZF(false);
    }

    fn RR(&mut self, value:u8) -> u8{
        let bit = 0b0000_0001 & value;
        let carry = if self.get_CF() {1} else {0};
        let result = (value >> 1) | (carry << 7);
        self.update_flags(result == 0, bit != 0, false, false);

        result
    }

    fn RRA(&mut self){
        self.registers.A = self.RR(self.registers.A);
        self.set_ZF(false);
    }

    fn SLA(&mut self, value:u8) -> u8{
        let seven = value >> 7 & 1 != 0;
        let result = value << 1;
        self.update_flags(result == 0, seven, false, false);
        result
    }

    fn SRA(&mut self, value:u8) -> u8{
        let seven = value & 0b1000_0000;
        let bit = value & 0b0000_0001;
        let result = (value >> 1) | seven;
        self.update_flags(result == 0, bit != 0, false, false);
        result
    }

    fn SRL(&mut self, value:u8) -> u8{
        let bit = value & 0b0000_0001;
        let result = value >> 1;
        self.update_flags(result == 0, bit != 0, false, false);
        result
    }

    fn fetch_byte(&mut self) -> u8 {
        let op = self.memory[self.registers.PC as usize];
        self.registers.PC += 1;
        op
    }

    fn fetch_word(&mut self) -> u16 {
        let op = (self.memory[self.registers.PC as usize] as u16)
            | ((self.memory[(self.registers.PC + 1) as usize] as u16) << 8);
        self.registers.PC += 2;
        op
    }
}

fn main() {
    let mut cpu = CPU::new();
}
