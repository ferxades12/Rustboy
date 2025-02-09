use crate::cpu::CPU;
use crate::op_codes::execute_opcode;

mod cpu;
mod op_codes;


fn main() {
    let mut cpu = CPU::new();

    // Load the ROM into memory


    // Set the program counter to the start of the ROM

    // Start the fetch-decode-execute cycle
    loop{
        if cpu.ei_flag{
            cpu.ei_flag = false;
            cpu.registers.IME = true;
        }

        handle_interrupts(&mut cpu); 


        execute_opcode(&mut cpu);

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