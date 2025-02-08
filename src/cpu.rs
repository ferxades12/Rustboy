
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
    pub IR: u8, // Instruction register
    pub IE: u8, // Interrupt Enable
    pub IF: u8, // Interrupt Flag
    pub IME: bool, // Interrupt master enable
}

pub enum InterruptCode { 
    Vblank = 0, 
    Lcd = 1, 
    Timer = 2,
    Serial = 3,
    Joypad = 4, 
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
            IE: 0, // Interrupt Enable // 7 6 5 Joypad Serial Timer LCD V-Blank    
            IF: 0, // Interrupt Flag (Requests an interrupt) // 7 6 5 Joypad Serial Timer LCD V-Blank
            IR: 0, // Instruction register
            IME: false,
        }
    }
}

pub struct CPU {
    pub registers: Registers,
    pub memory: [u8; MEMORY_SIZE], // Memoria de la CPU
    pub ei_flag:bool, // Flag de interrupciones
    pub stop_flag:bool, // Flag de parada 
    pub halt_flag:bool,

                               /*
                               FLAGS: Bits 7-4 de F

                               ZF:bool,    // Si es 0
                               NF:bool,    // Si es resta
                               HF:bool,    // Si hubo carry del bit 3 al 4
                               CF:bool,    // Si hay acarreo fuera de rango */
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            memory: [0; MEMORY_SIZE], // Inicializa la memoria a ceros
            ei_flag: false,
            stop_flag: false,
            halt_flag: false,
        }
    }

    pub fn get_ie(&self, code: InterruptCode) -> bool { 
        (self.registers.IE & (1 << code as u8)) != 0
    }
 
    pub fn get_if(&self, code: InterruptCode) -> bool { 
        (self.registers.IF & (1 << code as u8)) != 0
    }

    pub fn set_ie(&mut self, code: InterruptCode, value: bool) { 
        if value {
            self.registers.IE |= 1 << code as u8;
        } else {
            self.registers.IE &= !(1 << code as u8);
        }
    }

    pub fn set_if(&mut self, code: InterruptCode, value: bool) { 
        if value {
            self.registers.IF |= 1 << code as u8;
        } else {
            self.registers.IF &= !(1 << code as u8);
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

        self.update_flags(result == 0, carry1 | carry2, (n1 & 0x0F) + (n2 & 0x0F) + carry_prev > 0x0F, false);

        result
    }

    pub fn SBC(&mut self, n2: u8) -> u8 {
        let n1 = self.registers.A;
        let carry_prev = self.get_CF() as u8;
        let (result, carry1) = n1.overflowing_sub(n2);
        let (result, carry2) = result.overflowing_sub(carry_prev);

        self.update_flags(result == 0, carry1 | carry2, (n1 & 0x0F) < ((n2 & 0x0F) + carry_prev), true);

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

    pub fn CP(&mut self, num: u8) { // Compara. Comprueba la resta pero no guarda el resultado
        let (result, carry) = self.registers.A.overflowing_sub(num);
        self.update_flags(
            result == 0,
            carry,
            (self.registers.A & 0x0F) + (num & 0x0F) > 0x0F,
            true, 
        );
    }

    pub fn INC(&mut self, value:u8) -> u8 {
        let result = value.wrapping_add(1);
        self.update_flags(result as u8 == 0, self.get_CF(), (value & 0x0F) + 1 > 0x0F, false);

        result
    }

    pub fn DEC(&mut self, value:u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.update_flags(result as u8 == 0, self.get_CF(), (value & 0x0F) < 1, true);

        result
    }

    pub fn RLC(&mut self, value:u8) -> u8 { 
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

    pub fn RRC(&mut self, value:u8) -> u8{
        let bit = 0b0000_0001 & value;
        let result = (value >> 1) | (bit << 7);
        self.update_flags(result == 0, bit != 0, false, false);
        result
    }
    
    pub fn RRCA(&mut self) {
        self.registers.A = self.RRC(self.registers.A);
        self.set_ZF(false);
    }

    pub fn RR(&mut self, value:u8) -> u8{
        let bit = 0b0000_0001 & value;
        let carry = if self.get_CF() {1} else {0};
        let result = (value >> 1) | (carry << 7);
        self.update_flags(result == 0, bit != 0, false, false);

        result
    }

    pub fn RRA(&mut self){
        self.registers.A = self.RR(self.registers.A);
        self.set_ZF(false);
    }

    pub fn SLA(&mut self, value:u8) -> u8{
        let seven = value >> 7 & 1 != 0;
        let result = value << 1;
        self.update_flags(result == 0, seven, false, false);
        result
    }

    pub fn SRA(&mut self, value:u8) -> u8{
        let seven = value & 0b1000_0000;
        let bit = value & 0b0000_0001;
        let result = (value >> 1) | seven;
        self.update_flags(result == 0, bit != 0, false, false);
        result
    }

    pub fn SRL(&mut self, value:u8) -> u8{
        let bit = value & 0b0000_0001;
        let result = value >> 1;
        self.update_flags(result == 0, bit != 0, false, false);
        result
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let op = self.memory[self.registers.PC as usize];
        self.registers.PC += 1;
        op
    }

    pub fn fetch_word(&mut self) -> u16 {
        let op = (self.memory[self.registers.PC as usize] as u16)
            | ((self.memory[(self.registers.PC + 1) as usize] as u16) << 8);
        self.registers.PC += 2;
        op
    }

    pub fn JR (&mut self, condition: bool) {
        let offset:i8 = self.fetch_byte() as i8;
        if condition {
            self.registers.PC = (self.registers.PC as i32 + offset as i32) as u16;
        }
    }

    pub fn JP(&mut self,  condition: bool){
        let address = self.fetch_word();
        if condition {
            self.registers.PC = address;
        }
    }

    pub fn RES(&mut self, bit:u8, num:u8) -> u8 {
        num & !(1 << bit)
    }

    pub fn SET(&mut self, bit:u8, num:u8) -> u8{
        num | (1 << bit)
    }

    pub fn BIT(&mut self, bit:u8, num:u8){
        let res = (num & (1 << bit)) == 0;

        self.update_flags(res, self.get_CF(), true, false);
    }

    pub fn DAA(&mut self) { // Fumada que no se usa nunca. Generated by Chat-GPT
        let mut a = self.registers.A;
        let mut adjust = 0;
        let mut carry = self.get_CF();
    
        if self.get_CF() || a > 0x99 {
            adjust |= 0x60;
            carry = true; // CF debe activarse si se ajusta por 0x60
        }
        if self.get_HF() || (a & 0x0F) > 0x09 {
            adjust |= 0x06;
        }
    
        if !self.get_NF() {
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }
    
        self.update_flags(a == 0, carry, false, self.get_NF());
        self.registers.A = a;
    }
    
    pub fn CPL(&mut self){
        self.registers.A = !self.registers.A;

        self.set_NF(true);
        self.set_HF(true);
    }

    pub fn SWAP(&mut self, value:u8) -> u8{
        let low = value & 0x0F;
        let high = value & 0xF0;
        let res = (low << 4) | (high >> 4);
        self.update_flags(res == 0, false, false, false);
        res
    }

    pub fn CCF(&mut self){
        self.set_CF(!self.get_CF());
        self.set_NF(false);
        self.set_HF(false);
    }

    pub fn SCF(&mut self){
        self.set_CF(true);
        self.set_NF(false);
        self.set_HF(false);
    }
    
    pub fn POP(&mut self) -> u16 {
        let value = (self.memory[(self.registers.SP + 1) as usize] as u16) << 8
            | self.memory[self.registers.SP as usize] as u16;
        self.registers.SP += 2;
        value
    }

    pub fn PUSH(&mut self, value:u16){
        self.registers.SP -= 2;
        self.memory[self.registers.SP as usize] = (value & 0xFF) as u8;
        self.memory[(self.registers.SP + 1) as usize] = (value >> 8) as u8;
    }

    pub fn RST(&mut self, address: u16){
        self.PUSH(self.registers.PC.wrapping_add(1));
        self.registers.PC = address;
    }

    pub fn RET(&mut self, condition: bool){
        if condition{
            self.registers.PC = self.POP();
        }
    }

    pub fn CALL(&mut self, condition: bool){
        let address = self.fetch_word();

        if condition{
            self.PUSH(self.registers.PC);
            self.registers.PC = address;
        }
    }


}