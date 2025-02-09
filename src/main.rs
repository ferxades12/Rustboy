use crate::cpu::CPU;
use crate::op_codes::execute_opcode;

mod cpu;
mod op_codes;

const DIV_INCREMENT_RATE: u16 = 256; // 16384 Hz


fn main() {
    let mut cpu = CPU::new();

    // Load the ROM into memory


    // Set the program counter to the start of the ROM

    // Start the fetch-decode-execute cycle

    let mut cycle_counter:u16 = 0;
    let mut tima_counter:u16 = 0;

    loop{
        // Increment the DIV register
        if cycle_counter >= DIV_INCREMENT_RATE{
            cpu.registers.DIV = cpu.registers.DIV.wrapping_add(1);
            cycle_counter -= DIV_INCREMENT_RATE;
        }

        // Increment the TIMA register
        if tima_counter >= cpu.get_tac_frequency() && cpu.get_tac_enabled(){  
            tima_counter -= cpu.get_tac_frequency();
            let (result, overflow) = cpu.registers.TIMA.overflowing_add(1);

            if overflow{
                cpu.set_if(cpu::InterruptCode::Timer, true);
                cpu.registers.TIMA = cpu.registers.TMA;
            } else {
                cpu.registers.TIMA = result;
            }
        }

        if cpu.ei_flag{
            cpu.ei_flag = false;
            cpu.registers.IME = true;
        }

        handle_interrupts(&mut cpu); 


        let cycles = execute_opcode(&mut cpu) as u16;
        tima_counter += cycles;
        cycle_counter += cycles;

        if cpu.halt_flag {break;}
    }

}

fn handle_interrupts(cpu: &mut CPU){
    if cpu.registers.IME {
        if cpu.get_if(cpu::InterruptCode::Vblank) && cpu.get_ie(cpu::InterruptCode::Vblank){                         // Check both IME and IF
            cpu.registers.IME = false;                                           
            cpu.set_if(cpu::InterruptCode::Vblank, false);          // Unset IME and IF
            //cpu.nop() x2
            cpu.PUSH(cpu.registers.PC);                                         // Push the current program counter onto the stack
            cpu.registers.PC = 0x40;                                            // Jump to the interrupt handler


        }
        else if cpu.get_if(cpu::InterruptCode::Lcd)&& cpu.get_ie(cpu::InterruptCode::Lcd){
            cpu.registers.IME = false;
            cpu.set_if(cpu::InterruptCode::Lcd, false);
            //cpu.nop() x2
            cpu.PUSH(cpu.registers.PC);
            cpu.registers.PC = 0x48;


        }
        else if cpu.get_if(cpu::InterruptCode::Timer)&& cpu.get_ie(cpu::InterruptCode::Timer){
            cpu.registers.IME = false;
            cpu.set_if(cpu::InterruptCode::Timer, false);
            //cpu.nop() x2
            cpu.PUSH(cpu.registers.PC);
            cpu.registers.PC = 0x50;


        }
        else if cpu.get_if(cpu::InterruptCode::Serial)&& cpu.get_ie(cpu::InterruptCode::Serial){
            cpu.registers.IME = false;
            cpu.set_if(cpu::InterruptCode::Serial, false);
            //cpu.nop() x2
            cpu.PUSH(cpu.registers.PC);
            cpu.registers.PC = 0x58;


        }
        else if cpu.get_if(cpu::InterruptCode::Joypad)&& cpu.get_ie(cpu::InterruptCode::Joypad){
            cpu.registers.IME = false;
            cpu.set_if(cpu::InterruptCode::Joypad, false);
            //cpu.nop() x2
            cpu.PUSH(cpu.registers.PC);
            cpu.registers.PC = 0x60;


        }
    }
} 