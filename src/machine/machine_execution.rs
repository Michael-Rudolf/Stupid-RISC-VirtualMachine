use crate::machine::machine::Machine;
use crate::instruction;
use std::process::Command;
use std::time::Instant;

impl Machine {

    pub fn execute(&mut self, herz: Option<u32>){
        let start_time = Instant::now();
        while self.flags & 0x4000_0000 < 1 && self.execution_pointer < 100 {
            self.execute_line(herz);
        }
        let duration = Instant::now().duration_since(start_time);
        println!("Execution finished after: {:?}", duration);
    }
    pub fn execute_line(&mut self, herz: Option<u32>) {
        // Check halt bit & potentially avoid further execution
        if self.flags & 0x4000_0000 >= 1 { return; }

        // Fetch
        let instruction = self.memory[self.execution_pointer as usize];
        let operand_1 = self.memory[(self.execution_pointer + 1) as usize];
        let operand_2 = self.memory[(self.execution_pointer + 2) as usize];

        // Clean the terminal
        print!("{}[2J", 27 as char);
        // Print the current instruction
        println!("Executing: {:?}", instruction::Instruction::name_of_instruction(instruction, operand_1, operand_2).unwrap_or("nothing.".to_string()));
        // Print the registers
        self.print_registers();

        // Update the execution pointer
        self.execution_pointer += 3;

        // Get data from registers or pass the data from the operands
        let data_1 = if operand_1 >= 128 { self.data_of_register_by_value(operand_1 - 128) } else { operand_1 as i32 };
        let data_2 = if operand_2 >= 128 { self.data_of_register_by_value(operand_2 - 128) } else { operand_2 as i32 };

        // Create a result to update the register
        let mut result: Option<i32> = None;
        let mut ticks: u32 = 0;
        match instruction {
            instruction::ADD_INSTRUCTION => { result = Some(data_1 + data_2); ticks = 5 },
            instruction::SUB_INSTRUCTION => { result = Some(data_1 - data_2); ticks = 6 },
            instruction::MUL_INSTRUCTION => { result = Some(data_1 * data_2); ticks = 50 },
            instruction::DIV_INSTRUCTION => { result = Some(data_1 / data_2); ticks = 50 },
            instruction::MOD_INSTRUCTION => { result = Some(data_1 % data_2); ticks = 51 },
            instruction::MOVE_INSTRUCTION => { result = Some(data_2); ticks = 3 },
            instruction::HALT_INSTRUCTION => { result = Some(data_1 | 0x4000_0000); ticks = 4 }, // Set halt bit
            instruction::JUMP_INSTRUCTION => { result = Some(data_2); ticks = 4 }, // Load the second input to the register in (Exec ptr register) and multiply by 3 to get from the line to the actual memory address.
            instruction::JUMP_ZERO_INSTRUCTION => {
                if data_1 == 0 {
                    self.execution_pointer = (data_2) as u32;
                }
                ticks = 5;
            }
            instruction::LOAD_BYTE_INSTRUCTION => { result = Some(self.memory[data_2 as usize] as i32); ticks = 5 },
            instruction::STORE_BYTE_INSTRUCTION => { self.memory[data_2 as usize] = data_1 as u8; ticks = 5 },
            _=> result = None
        }

        if let Some(herz) = herz {
            let wait_time_s = 1. / (herz as f32) * ticks as f32;
            let mut command = Command::new("sleep").arg(wait_time_s.to_string()).spawn().unwrap();
            let _result = command.wait().unwrap();
        }

        if operand_1 >= 128 && result != None{
            self.set_data_of_register(operand_1 - 128, result.unwrap());
        }
    }
}