use std::result;

const MEMORY_SIZE: usize = 65536;
const ROM_BANK_0: usize = 0x0000;  // ROM Bank 0 (32KB)
const ROM_BANK_1: usize = 0x4000;  // ROM Bank 1 (32KB)
const VRAM: usize = 0x8000;        // VRAM (8KB)
const RAM_BANK_0: usize = 0xC000;  // RAM Bank 0 (8KB)
const OAM: usize = 0xFE00;         // OAM (Sprites) (160 bytes)
const IO_REGISTERS: usize = 0xFF00; // IO Registros (80 bytes)
const HIGH_RAM: usize = 0xFF80;    // Memoria de alto rendimiento (128 bytes)

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
enum Register16{
    PC,
    SP
}

// Enum para las parejas de registros
#[derive(Copy, Clone)]
enum RegisterPair {
    HL,
    DE,
    BC,
}

trait Operand<T>{
    fn write(&self, cpu: &mut CPU, value: T);
    fn read(&self, cpu: &CPU) -> T;
}

impl Operand<usize> for Register8{
    fn read(&self, cpu: &CPU) -> usize {
        panic!("No puedes leer un usize de un Reg8")
    }

    fn write(&self, cpu: &mut CPU, value: usize) {
        cpu.set_register8(*self, cpu.memory[value]);
    }
}

impl Operand<usize> for Register16{
    fn read(&self, cpu: &CPU) -> usize {
        panic!("No puedes leer un usize de un Reg")
    }

    fn write(&self, cpu: &mut CPU, value: usize) {
        let result = (cpu.memory[value + 1] as u16) << 8 | cpu.memory[value] as u16;
        cpu.set_register16(*self, result);
    }
}


impl Operand<u8> for Register8{
    fn read(&self, cpu: &CPU) -> u8 {
        cpu.get_register8(*self)
    }

    fn write(&self, cpu: &mut CPU, value: u8) {
        cpu.set_register8(*self, value);
    }
}


impl Operand<Register8> for usize{
    fn read(&self, cpu: &CPU) -> Register8 {
        panic!("No puedes leer un Reg8 de un usize")
    }

    fn write(&self, cpu: &mut CPU, reg: Register8) {
        cpu.memory[*self] = cpu.get_register8(reg);
    }
}

impl Operand<Register8> for RegisterPair{
    fn read(&self, cpu: &CPU) -> Register8 {
        panic!("No puedes leer un Reg8 de un RegPair")
    }

    fn write(&self, cpu: &mut CPU, reg: Register8) {
        cpu.memory[cpu.get_register_pair(*self) as usize] = cpu.get_register8(reg);
    }
}

impl Operand<u8> for RegisterPair{
    fn read(&self, cpu: &CPU) -> u8 {
        panic!("No puedes leer un u8 de un RegPair")
    }

    fn write(&self, cpu: &mut CPU, value: u8) {
        cpu.memory[cpu.get_register_pair(*self) as usize] = value;
    }
}

impl Operand<RegisterPair> for Register8{
    fn read(&self, cpu: &CPU) -> RegisterPair {
        panic!("No puedes leer un RegisterPair de un Reg8")
    }

    fn write(&self, cpu: &mut CPU, pair: RegisterPair) {
        cpu.set_register8(*self, cpu.memory[cpu.get_register_pair(pair) as usize]);
    }
}

impl Operand<u16> for Register16{
    fn read(&self, cpu: &CPU) -> u16 {
        cpu.get_register16(*self)
    }

    fn write(&self, cpu: &mut CPU, value: u16) {
        cpu.set_register16(*self, value);
    }
}

impl Operand<Register8> for Register8{
    fn read(&self, cpu: &CPU) -> Register8 {
        panic!()
    }

    fn write(&self, cpu: &mut CPU, value: Register8) {
        cpu.set_register8(*self, cpu.get_register8(value));
    }
}

impl Operand<u16> for RegisterPair{
    fn read(&self, cpu: &CPU) -> u16 {
        cpu.get_register_pair(*self)
    }

    fn write(&self, cpu: &mut CPU, value: u16) {
        cpu.set_register_pair(*self, value);
    }
}





struct CPU {
    PC: u16,        // Program counter (16bit)
    SP: u16,        // Stack pointer
    A: u8,          // Accumulator
    F: u8,          // Flags register. Los bits 0-3 estan a 0 y no se usan 

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

impl CPU{
    fn new() -> CPU {
        CPU {
            PC: 0100,
            SP: 0,
            A: 0,
            B: 0,
            C: 0,
            D:0, 
            E: 0,
            H: 0,
            L:0,
            IR: 0,
            IE: 0,
            F:0,
            memory: [0; MEMORY_SIZE], // Inicializa la memoria a cero
        }
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
            Register16::SP => self.SP
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
            Register16::SP => self.SP = value
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

    fn LD<T>(&mut self, to: impl Operand<T>, from: impl Operand<T>){
        let value = from.read(self);
        to.write(self, value);
    }  

    fn POP(&mut self, To: RegisterPair){
        let value = (self.memory[(self.SP + 1) as usize] as u16) << 8 | self.memory[self.SP as usize] as u16;
        self.set_register_pair(To, value);
        self.SP += 2 ;
    }

    fn update_flags(&mut self, zero: bool, carry:bool, half_carry:bool, substract:bool){
        self.F = 0;
        if zero {self.F |= 0b1000_0000;}
        if substract {self.F |= 0b0100_0000;}
        if half_carry {self.F |= 0b0010_0000;}
        if carry {self.F |= 0b0001_0000;}
    }

    fn ADD<T>(&mut self, op: impl Operand<T>) where T: Into<u8> {
        let num = op.read(self).into();
        let result = self.A as u16 + num as u16;

        self.update_flags(result == 0, result > 0xFF,(self.A & 0x0F) + (num & 0x0F) > 0x0F , false);
        self.A = result as u8;
    }

    
    fn SUB(&mut self, num:u8){
        let result = self.A as u16 - num as u16;
  
        self.update_flags(result == 0, result > 0xFF, (self.A & 0x0F) < (num & 0x0F), true); 

        self.A = result as u8;
    }

    fn SUB_register8(&mut self, reg: Register8){
        self.SUB(self.get_register8(reg));
    }

    fn SUB_indirect_adress(&mut self, reg: RegisterPair){
        let adress = self.get_register_pair(reg) as usize;
        self.SUB(self.memory[adress]);
    }

    fn ADC(&mut self, num: u8) {
        let carry:u8 = if (self.F & 0b0001_0000) != 0 { 1 } else { 0 };
        //self.ADD(num + carry);
    }

    fn ADC_register8(&mut self, reg: Register8) {
        self.ADC(self.get_register8(reg));
    }

    fn ADC_indirect_adress(&mut self, reg: RegisterPair) {
        let adress = self.get_register_pair(reg) as usize;
        self.ADC(self.memory[adress]);
    }

    fn SBC(&mut self, num: u8){
        let carry:u8 = if (self.F & 0b0001_0000) != 0 { 1 } else { 0 };
        self.SUB(num + carry);
    }

    fn SBC_register8(&mut self, reg: Register8) {
        self.SBC(self.get_register8(reg));
    }

    fn SBC_indirect_adress(&mut self, reg: RegisterPair) {
        let adress = self.get_register_pair(reg) as usize;
        self.SBC(self.memory[adress]);
    }

    //TODO
    fn AND(&mut self, num: u8){ 
        self.A &= num;
        self.update_flags(self.A == 0, false, true, false);
    }

    fn OR (&mut self, num: u8){
        self.A |= num;
        self.update_flags(self.A == 0, false, false, false);
    }

    fn XOR (&mut self, num: u8){
        self.A ^= num;
        self.update_flags(self.A == 0, false, false, false);
    }

    fn CP(&mut self, num:u8){
        let result = self.A as u16 - num as u16;
        self.update_flags(result == 0, result > 0xFF,(self.A & 0x0F) + (num & 0x0F) > 0x0F , false);
    }

    fn Inc(&mut self, reg: Register8){
        let result = self.get_register8(reg) as u16 + 1;

        self.update_flags(result == 0, false, (self.get_register8(reg) & 0x0F) + 1 > 0x0F, false);
    }

    fn DEC (&mut self, reg: Register8){
        let result = self.get_register8(reg) as u16 - 1;
        self.update_flags(result == 0, false, (self.get_register8(reg) & 0x0F) < 1, true);
    }

}

fn main() {
    let mut cpu = CPU::new();
}
