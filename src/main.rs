const MEMORY_SIZE: usize = 65536;
const ROM_BANK_0: usize = 0x0000;  // ROM Bank 0 (32KB)
const ROM_BANK_1: usize = 0x4000;  // ROM Bank 1 (32KB)
const VRAM: usize = 0x8000;        // VRAM (8KB)
const RAM_BANK_0: usize = 0xC000;  // RAM Bank 0 (8KB)
const OAM: usize = 0xFE00;         // OAM (Sprites) (160 bytes)
const IO_REGISTERS: usize = 0xFF00; // IO Registros (80 bytes)
const HIGH_RAM: usize = 0xFF80;    // Memoria de alto rendimiento (128 bytes)

// Enum para los registros individuales
enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

enum Register16{
    PC,
    SP
}

// Enum para las parejas de registros
enum RegisterPair {
    HL,
    DE,
    BC,
}
struct CPU {
    PC: u16,        // Program counter (16bit)
    SP: u16,        // Stack pointer
    A: u8,          // Accumulator
    F: u8,          // Flags register   

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

    fn LD_reg8_to_reg8(&mut self, To: Register8, From: Register8){
        self.set_register8(To, self.get_register8(From));
    }

    fn LD_value_to_reg8(&mut self, To: Register8, From: u8){
        self.set_register8(To, From);
    }

    fn LD_value_to_reg16(&mut self, To: Register16, From: u16){
        self.set_register16(To, From);
    }

    fn LD_value_to_pair(&mut self, To: RegisterPair, From: u16){
        self.set_register_pair(To, From);
    }

    fn LD_adress_to_reg8(&mut self, To: Register8, From: usize){
        self.set_register8(To, self.memory[From]);
    }

    fn LD_adress_to_reg16(&mut self, To: Register16, From: usize){
        let value = (self.memory[From + 1] as u16) << 8 | self.memory[From] as u16;
        self.set_register16(To, value);
    }

    fn LD_indirect_adress_to_reg8(&mut self, To: Register8, From: RegisterPair){
        let address = self.get_register_pair(From) as usize;
        self.LD_adress_to_reg8(To, address);
    }

    fn POP(&mut self, To: RegisterPair){
        let value = (self.memory[(self.SP + 1) as usize] as u16) << 8 | self.memory[self.SP as usize] as u16;
        self.set_register_pair(To, value);
        self.SP += 2 ;
    }

    fn LD_reg8_to_adress(&mut self, To: usize, From: Register8){
        self.memory[To] = self.get_register8(From);
    }

    fn LD_reg8_to_indirect_adress(&mut self, To: RegisterPair, From: Register8){
        self.LD_reg8_to_adress(self.get_register_pair(To) as usize, From);
    }

    fn LD_value_to_indirect_adress(&mut self, To: RegisterPair, From: u8){
        let adress = self.get_register_pair(To) as usize;
        self.memory[adress] = From;
    }

    

}

fn main() {
    let mut cpu = CPU::new();
}
