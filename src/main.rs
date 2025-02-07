mod opCodes;


const MEMORY_SIZE: usize = 65536;
const ROM_BANK_0: usize = 0x0000; // ROM Bank 0 (32KB) HOME BANK
const ROM_BANK_1: usize = 0x4000; // ROM Bank 1 (32KB)
const VRAM: usize = 0x8000; // VRAM (8KB) Background tiles
const CARTRIDGE_RAM:usize = 0xA000;
const WORK_RAM: usize = 0xC000; // RAM Bank 0 (8KB)
// Space not used
const OAM: usize = 0xFE00; // OAM (Sprites) (160 bytes) also tiles
//Space not used
const IO_REGISTERS: usize = 0xFF00; // IO Registros (80 bytes)
const HIGH_RAM: usize = 0xFF80; // Memoria de alto rendimiento (128 bytes) //Acceso un ciclo mas rapido

// Enum para los registros individuales
#[derive(Copy, Clone)]
enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
#[derive(Copy, Clone)]
enum Register16 {
    PC,
    SP,
}

// Enum para las parejas de registros
#[derive(Copy, Clone)]
enum RegisterPair {
    HL,
    DE,
    BC,
}
trait AllowedTypes: Into<u16> + Copy {}
impl AllowedTypes for u8 {}
impl AllowedTypes for u16 {}
trait Operand<T: AllowedTypes> {
    fn write(&self, cpu: &mut CPU, value: T);
    fn read(&self, cpu: &CPU) -> T;
}

impl Operand<u8> for u8{
    fn read(&self, _cpu: &CPU) -> u8 {
        *self
    }

    fn write(&self, _cpu: &mut CPU, _value: u8) {
        panic!("No puedes escribir en un u8")
    }
}

impl Operand<u8> for Register8 {
    fn read(&self, cpu: &CPU) -> u8 {
        cpu.get_register8(*self)
    }

    fn write(&self, cpu: &mut CPU, value: u8) {
        cpu.set_register8(*self, value);
    }
}

impl Operand<u8> for usize {
    fn read(&self, cpu: &CPU) -> u8 {
        cpu.memory[*self]
    }

    fn write(&self, cpu: &mut CPU, value: u8) {
        cpu.memory[*self] = value;
    }
}

impl Operand<u16> for Register16 {
    fn read(&self, cpu: &CPU) -> u16 {
        cpu.get_register16(*self)
    }

    fn write(&self, cpu: &mut CPU, value: u16) {
        cpu.set_register16(*self, value);
    }
}

impl Operand<u16> for u16 {
    fn read(&self, _cpu: &CPU) -> u16 {
        *self
    }

    fn write(&self, _cpu: &mut CPU, _value: u16) {
        panic!("No puedes escribir en un u16")
    }
}

impl Operand<u8> for RegisterPair {
    fn read(&self, _cpu: &CPU) -> u8 {
        panic!("No puedes leer un u8 de un RegPair")
    }

    fn write(&self, cpu: &mut CPU, value: u8) {
        cpu.memory[cpu.get_register_pair(*self) as usize] = value;
    }
}

impl Operand<u16> for RegisterPair {
    fn read(&self, cpu: &CPU) -> u16 {
        cpu.get_register_pair(*self)
    }

    fn write(&self, cpu: &mut CPU, value: u16) {
        cpu.set_register_pair(*self, value);
    }
} 

/*
impl Operand<usize> for Register8 {
    fn read(&self, _cpu: &CPU) -> usize {
        panic!("No puedes leer un usize de un Reg8")
    }

    fn write(&self, cpu: &mut CPU, value: usize) {
        cpu.set_register8(*self, cpu.memory[value]);
    }
}

impl Operand<Register8> for Register8 {
    fn read(&self, _cpu: &CPU) -> Register8 {
        *self
    }

    fn write(&self, cpu: &mut CPU, value: Register8) {
        cpu.set_register8(*self, cpu.get_register8(value));
    }
}


impl Operand<RegisterPair> for Register8 {
    fn read(&self, _cpu: &CPU) -> RegisterPair {
        panic!("No puedes leer un RegisterPair de un Reg8")
    }

    fn write(&self, cpu: &mut CPU, pair: RegisterPair) {
        cpu.set_register8(*self, cpu.memory[cpu.get_register_pair(pair) as usize]);
    }
}

impl Operand<Register8> for usize {
    fn read(&self, _cpu: &CPU) -> Register8 {
        panic!("No puedes leer un Reg8 de un usize")
    }

    fn write(&self, cpu: &mut CPU, reg: Register8) {
        cpu.memory[*self] = cpu.get_register8(reg);
    }
}


impl Operand<Register8> for RegisterPair {
    fn read(&self, _cpu: &CPU) -> Register8 {
        panic!("No puedes leer un Reg8 de un RegPair")
    }

    fn write(&self, cpu: &mut CPU, reg: Register8) {
        cpu.memory[cpu.get_register_pair(*self) as usize] = cpu.get_register8(reg);
    }
}

impl Operand<usize> for Register16 {
    fn read(&self, _cpu: &CPU) -> usize {
        panic!("No puedes leer un usize de un Reg")
    }

    fn write(&self, cpu: &mut CPU, value: usize) {
        let result = (cpu.memory[value + 1] as u16) << 8 | cpu.memory[value] as u16;
        cpu.set_register16(*self, result);
    }
}
*/

pub struct CPU {
    PC: u16, // Program counter (16bit)
    SP: u16, // Stack pointer
    A: u8,   // Accumulator
    F: u8,   // Flags register. Los bits 0-3 estan a 0 y no se usan

    // General purpose registers
    B: u8,
    C: u8,

    D: u8,
    E: u8,

    H: u8,
    L: u8,

    IR: u8, // Instruction register
    IE: u8, // Interrupt enable
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
            PC: 0100,
            SP: 0,
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            H: 0,
            L: 0,
            IR: 0,
            IE: 0,
            F: 0,
            memory: [0; MEMORY_SIZE], // Inicializa la memoria a ceros
        }
    }

    fn NOP(&mut self) {
        // No hace nada
    }
    // Accede a un registro individual
    fn get_register8(&self, reg: Register8) -> u8 {
        match reg {
            Register8::A => self.A,
            Register8::B => self.B,
            Register8::C => self.C,
            Register8::D => self.D,
            Register8::E => self.E,
            Register8::H => self.H,
            Register8::L => self.L,
        }
    }

    fn get_register16(&self, reg: Register16) -> u16 {
        match reg {
            Register16::PC => self.PC,
            Register16::SP => self.SP,
        }
    }

    // Accede a una pareja de registros
    fn get_register_pair(&self, pair: RegisterPair) -> u16 {
        match pair {
            RegisterPair::HL => ((self.H as u16) << 8) | (self.L as u16),
            RegisterPair::DE => ((self.D as u16) << 8) | (self.E as u16),
            RegisterPair::BC => ((self.B as u16) << 8) | (self.C as u16),
        }
    }

    // Modifica un registro individual
    fn set_register8(&mut self, reg: Register8, value: u8) {
        match reg {
            Register8::A => self.A = value,
            Register8::B => self.B = value,
            Register8::C => self.C = value,
            Register8::D => self.D = value,
            Register8::E => self.E = value,
            Register8::H => self.H = value,
            Register8::L => self.L = value,
        }
    }

    fn set_register16(&mut self, reg: Register16, value: u16) {
        match reg {
            Register16::PC => self.PC = value,
            Register16::SP => self.SP = value,
        }
    }
    // Modifica una pareja de registros
    fn set_register_pair(&mut self, pair: RegisterPair, value: u16) {
        match pair {
            RegisterPair::HL => {
                self.H = (value >> 8) as u8;
                self.L = (value & 0xFF) as u8;
            }
            RegisterPair::DE => {
                self.D = (value >> 8) as u8;
                self.E = (value & 0xFF) as u8;
            }
            RegisterPair::BC => {
                self.B = (value >> 8) as u8;
                self.C = (value & 0xFF) as u8;
            }
        }
    }

    fn LD<T:AllowedTypes>(&mut self, to: impl Operand<T>, from: impl Operand<T>) {
        let value = from.read(self);
        to.write(self, value);
    }

    fn POP(&mut self, To: RegisterPair) {
        let value = (self.memory[(self.SP + 1) as usize] as u16) << 8
            | self.memory[self.SP as usize] as u16;
        self.set_register_pair(To, value);
        self.SP += 2;
    }

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

    fn ADD<T>(&mut self, op: impl Operand<u8>){
        let num: u8 = op.read(self).into();
        let result = self.A as u16 + num as u16;

        self.update_flags(
            result == 0,
            result > 0xFF,
            (self.A & 0x0F) + (num & 0x0F) > 0x0F,
            false,
        );
        self.A = result as u8;
    }

    fn SUB<T>(&mut self, op: impl Operand<u8>)
    {
        let num:u8 = op.read(self).into();
        let result = self.A as u16 - num as u16;

        self.update_flags(
            result == 0,
            result > 0xFF,
            (self.A & 0x0F) < (num & 0x0F),
            true,
        );

        self.A = result as u8;
    }

    fn ADC<T>(&mut self, op: impl Operand<u8>)
    {
        let num:u8 = op.read(self).into();
        let carry: u8 = if (self.F & 0b0001_0000) != 0 { 1 } else { 0 };
        let result = self.A as u16 + num as u16 + carry as u16;

        self.update_flags(
            result == 0,
            result > 0xFF,
            (self.A & 0x0F) + (num & 0x0F) + carry > 0x0F,
            false,
        );
        self.A = result as u8;
    }

    fn SBC<T>(&mut self, op: impl Operand<u8>)
    {
        let num:u8 = op.read(self).into();
        let carry: u8 = if (self.F & 0b0001_0000) != 0 { 1 } else { 0 };
        let result = self.A as u16 - num as u16 - carry as u16;

        self.update_flags(
            result == 0,
            result > 0xFF,
            (self.A & 0x0F) < (num & 0x0F) + carry,
            true,
        );
        self.A = result as u8;
    }

    fn AND<T:AllowedTypes>(&mut self, op: impl Operand<u8>) {
        self.A &= op.read(self);
        self.update_flags(self.A == 0, false, true, false);
    }

    fn OR(&mut self, op: impl Operand<u8>) {
        self.A |= op.read(self);
        self.update_flags(self.A == 0, false, false, false);
    }

    fn XOR(&mut self, op: impl Operand<u8>) {
        self.A ^= op.read(self);
        self.update_flags(self.A == 0, false, false, false);
    }

    fn CP(&mut self, op: impl Operand<u8>) { // Compara. Comprueba la resta pero no guarda el resultado
        let result = self.A as u16 - op.read(self) as u16;
        self.update_flags(
            result == 0,
            result > 0xFF,
            (self.A & 0x0F) + (op.read(self) & 0x0F) > 0x0F,
            false,
        );
    }

    fn Inc<T>(&mut self, op: impl Operand<u8>) {
        let value = op.read(self);
        let result = value as u16 + 1;

        self.update_flags(
            result as u8 == 0,
            false,
            (value & 0x0F) + 1 > 0x0F,
            false,
        );

        op.write(self, result as u8);
    }

    fn DEC<T>(&mut self, op: impl Operand<u8>) {
        let value = op.read(self);
        let result = value as u16 - 1;

        self.update_flags(
            result as u8 == 0,
            false,
            (value & 0x0F) + 1 > 0x0F,
            false,
        );

        op.write(self, result as u8);
    }

    fn RLCA(&mut self){ // Mueve el bit 7 de A al bit 0 y al bit de carry
        self.RLC::<u8>(Register8::A);
    }

    fn RLC<T>(&mut self, op: impl Operand<u8>){
        let value = op.read(self);
        let seven = value >> 7 & 1 != 0;

        let new_value = (value << 1) | (seven as u8);
        op.write(self, new_value);
        self.update_flags(value == 0, seven, false, false);
    }

    fn RL<T>(&mut self, op: impl Operand<u8>){
        let value = op.read(self);
        let seven = value >> 7 & 1 != 0;
        let carry = (self.F & 0b0001_0000) >> 4;

        let new_value = (value << 1) | carry;
        op.write(self, new_value);
        self.update_flags(value == 0, seven, false, false);
    }

    fn RLA(&mut self){ // Mueve el bit 7 de a al carry y el carry al 0
        self.RL::<u8>(Register8::A);
    }

    fn fetch_byte(&mut self)-> u8{
        let op = self.memory[self.PC as usize];
        self.PC += 1;
        op
    }

    fn fetch_word(&mut self)-> u16{
        let op = (self.memory[self.PC as usize] as u16) | ((self.memory[(self.PC + 1) as usize] as u16) << 8);
        self.PC += 2;
        op
    }

}

fn main() {
    let mut cpu = CPU::new();
}
