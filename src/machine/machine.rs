pub struct Machine {
    pub memory: [u8; 4096],
    pub general_registers: [i32; 16-4],
    // HALT
    pub flags: u32,
    pub stack_pointer: u32,
    pub frame_pointer: u32,
    pub execution_pointer: u32,
}

impl Machine {
    pub fn new() -> Machine {
        let memory: [u8; 4096] = [0; 4096];
        let general_registers: [i32; 12] = [0; 12];
        Machine{memory, general_registers, flags: 0, stack_pointer: 0, frame_pointer: 0, execution_pointer: 0}
    }

    pub fn set_ram(&mut self, address: usize, data: Vec<u8>) {
        for i in 0..data.len(){
            self.memory[address + i] = data[i];
        }
    }

    pub fn data_of_register_by_value(&mut self, register: u8) -> i32 {
        if register > 15 { panic!() }
        match register {
            12 => self.flags as i32,
            13 => self.stack_pointer as i32,
            14 => self.frame_pointer as i32,
            15 => self.execution_pointer as i32,
            _ => self.general_registers[register as usize],
        }
    }

    pub fn set_data_of_register(&mut self, register: u8, value: i32) {
        if register > 15 { panic!() }
        match register {
            12 => self.flags = value as u32,
            13 => self.stack_pointer = value as u32,
            14 => self.frame_pointer = value as u32,
            15 => self.execution_pointer = value as u32,
            _ => self.general_registers[register as usize] = value,
        }
    }

    pub fn print_registers(&self) {
        for i in 1..self.general_registers.len()+1 {
            print!("R{:02}:      0x{:08x}      ", i - 1, self.general_registers[i - 1]);

            if i % 4 == 0 { print!("\n") }
        }

        println!("FLAGS:    0x{:08X}      EXE_PTR:  0x{:08x}      FRA_PTR:  0x{:08x}      STK_PTR:  0x{:08x}", self.flags, self.execution_pointer, self.frame_pointer, self.stack_pointer);
    }
}