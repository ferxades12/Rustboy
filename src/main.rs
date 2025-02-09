use crate::cpu::CPU;
use crate::op_codes::execute_opcode;

mod cpu;
mod op_codes;

const DIV_INCREMENT_RATE: u16 = 256; // 16384 Hz

fn main() {
    let mut cpu = CPU::new();

    // Load the ROM into memory

    // Set initial values for registers

    // Start the fetch-decode-execute cycle

    let mut cycle_counter: u16 = 0;
    let mut tima_counter: u16 = 0;

    loop {
        // Increment the DIV register
        if cycle_counter >= DIV_INCREMENT_RATE {
            cpu.registers.DIV = cpu.registers.DIV.wrapping_add(1); // Increment the DIV register
            cycle_counter -= DIV_INCREMENT_RATE; // Reset the cycle counter
        }

        // Increment the TIMA register
        if tima_counter >= cpu.get_tac_frequency() && cpu.get_tac_enabled() {
            // Checks overflow in TIMA and enable bit in TAC
            tima_counter -= cpu.get_tac_frequency(); // Reset the TIMA counter
            let (result, overflow) = cpu.registers.TIMA.overflowing_add(1);

            if overflow {
                // Requesti interrupt and reset TIMA
                cpu.set_if(cpu::InterruptCode::Timer, true);
                cpu.registers.TIMA = cpu.registers.TMA;
            } else {
                // Increment TIMA
                cpu.registers.TIMA = result;
            }
        }

        let cycles = handle_interrupts(&mut cpu);
        tima_counter += cycles;
        cycle_counter += cycles;

        let cycles = execute_opcode(&mut cpu) as u16;
        tima_counter += cycles;
        cycle_counter += cycles;

        if cpu.halt_flag {
            break;
        }
    }
}

fn handle_interrupts(cpu: &mut CPU) -> u16 {
    if cpu.ei_flag {
        cpu.ei_flag = false;
        cpu.registers.IME = true;
    }

    if cpu.registers.IME {
        if cpu.get_if(cpu::InterruptCode::Vblank) && cpu.get_ie(cpu::InterruptCode::Vblank) {
            // Check both IME and IF
            cpu.registers.IME = false;
            cpu.set_if(cpu::InterruptCode::Vblank, false); // Unset IME and IF
            cpu.PUSH(cpu.registers.PC); // Push the current program counter onto the stack
            cpu.registers.PC = 0x40; // Jump to the interrupt handler
            return 5;
        } else if cpu.get_if(cpu::InterruptCode::Lcd) && cpu.get_ie(cpu::InterruptCode::Lcd) {
            cpu.registers.IME = false;
            cpu.set_if(cpu::InterruptCode::Lcd, false);
            //cpu.nop() x2
            cpu.PUSH(cpu.registers.PC);
            cpu.registers.PC = 0x48;
            return 5;
        } else if cpu.get_if(cpu::InterruptCode::Timer) && cpu.get_ie(cpu::InterruptCode::Timer) {
            cpu.registers.IME = false;
            cpu.set_if(cpu::InterruptCode::Timer, false);
            //cpu.nop() x2
            cpu.PUSH(cpu.registers.PC);
            cpu.registers.PC = 0x50;
            return 5;
        } else if cpu.get_if(cpu::InterruptCode::Serial) && cpu.get_ie(cpu::InterruptCode::Serial) {
            cpu.registers.IME = false;
            cpu.set_if(cpu::InterruptCode::Serial, false);
            //cpu.nop() x2
            cpu.PUSH(cpu.registers.PC);
            cpu.registers.PC = 0x58;
            return 5;
        } else if cpu.get_if(cpu::InterruptCode::Joypad) && cpu.get_ie(cpu::InterruptCode::Joypad) {
            cpu.registers.IME = false;
            cpu.set_if(cpu::InterruptCode::Joypad, false);
            //cpu.nop() x2
            cpu.PUSH(cpu.registers.PC);
            cpu.registers.PC = 0x60;
            return 5;
        }
    }
    return 0;
}
