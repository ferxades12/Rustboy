use std::{fs::File, io::Read};

const MEMORY_SIZE: usize = 65536;
const ROM_BANK_0: usize = 0x0000; // ROM Bank 0 (32KB) HOME BANK
const ROM_BANK_1: usize = 0x4000; // ROM Bank 1 (32KB)
const VRAM: usize = 0x8000; // VRAM (8KB). $8000-$97FF
const CARTRIDGE_RAM: usize = 0xA000;
const WORK_RAM: usize = 0xC000; // RAM Bank 0 (8KB)
                                // Space not used
const OAM: usize = 0xFE00; // OAM (Sprites) (160 bytes) also tiles
                           //Space not used
const IO_REGISTERS: usize = 0xFF00; // IO Registros (80 bytes)
const HIGH_RAM: usize = 0xFF80; // Memoria de alto rendimiento (128 bytes) //Acceso un ciclo mas rapido

pub struct MMU {
    pub memory: [u8; MEMORY_SIZE], // Memoria de la CPU
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        // Rom test
        if address == 0xFF01 && self.memory[0xFF02] == 0x81 {
            print!("{}", value as char);
            self.memory[0xFF02] = 0x00;
        }

        // Divider register
        if address == 0xFF04 {
            self.memory[address as usize] = 0;
            return;
        }

        // ROM BANK 0
        if address < VRAM as u16 {
            return;
        }

        self.memory[address as usize] = value;
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low_byte = self.read_byte(address) as u16;
        let high_byte = self.read_byte(address.wrapping_add(1)) as u16;

        (high_byte << 8) | low_byte
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let low_byte = value as u8;
        let high_byte = (value >> 8) as u8;

        self.write_byte(address, low_byte);
        self.write_byte(address + 1, high_byte);
    }

    pub fn read_rom(&mut self, file_path: &str) {
        let mut file = File::open(file_path).expect("Error al abrir la ROM");
        file.read(&mut self.memory[..])
            .expect("Error al cargar la ROM en memoria");
    }
}
