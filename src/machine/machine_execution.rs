use crate::machine::machine::Machine;
use crate::instruction;
use std::process::Command;
use std::time::Instant;
use colored::Colorize;

impl Machine {

    pub fn execute(&mut self, herz: Option<u32>){
        let start_time = Instant::now();
        while self.flags & 0x4000_0000 < 1 && self.execution_pointer < 100 {
            self.execute_line(herz);
        }
        let duration = Instant::now().duration_since(start_time);
        println!("{}", format!("Execution finished after: {:?}", duration).green());
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
        println!("Executing: {:?}", instruction::Instruction::name_of_instruction(instruction, operand_1, operand_2).unwrap_or("nothing.".yellow().to_string()));
        // Print the registers
        self.print_registers();
        println!("{}", self.standard_output);

        let mut execution_pointer_inc: u32 = 0;


        // Get data from registers or pass the data from the operands
        let data_1 = if operand_1 >= 128 { self.data_of_register_by_value(operand_1 - 128) } else { operand_1 as i32 };
        let data_2 = if operand_2 >= 128 { self.data_of_register_by_value(operand_2 - 128) } else { operand_2 as i32 };

        // Create a result to update the register
        let mut result: Option<i32> = None;
        let mut ticks: u32 = 0;
        match instruction {
            instruction::ADD_INSTRUCTION => { result = Some(data_1 + data_2); ticks = 5; execution_pointer_inc = 3; },
            instruction::SUB_INSTRUCTION => { result = Some(data_1 - data_2); ticks = 6; execution_pointer_inc = 3; },
            instruction::MUL_INSTRUCTION => { result = Some(data_1 * data_2); ticks = 50; execution_pointer_inc = 3; },
            instruction::DIV_INSTRUCTION => { result = Some(data_1 / data_2); ticks = 50; execution_pointer_inc = 3; },
            instruction::MOD_INSTRUCTION => { result = Some(data_1 % data_2); ticks = 51; execution_pointer_inc = 3; },
            instruction::MOVE_INSTRUCTION => { result = Some(data_2); ticks = 3; execution_pointer_inc = 3; },
            instruction::HALT_INSTRUCTION => { self.flags = self.flags | 0x4000_0000; ticks = 4; execution_pointer_inc = 100; }, // Set halt bit
            instruction::JUMP_INSTRUCTION => { self.execution_pointer = data_1 as u32; ticks = 4; execution_pointer_inc = 0; }, // Load the second input to the register in (Exec ptr register) and multiply by 3 to get from the line to the actual memory address.
            instruction::JUMP_ZERO_INSTRUCTION => {
                if data_1 == 0 {
                    self.execution_pointer = (data_2 - 3) as u32;
                }
                ticks = 5;
                execution_pointer_inc = 3;
            }
            instruction::PUSH_BYTE_INSTRUCTION => {
                self.memory[self.stack_pointer as usize] = data_2 as u8;
                self.stack_pointer -= 1;
                ticks = 9;
                execution_pointer_inc = 2;
            },
            instruction::POP_BYTE_INSTRUCTION => {
                result = Some(self.memory[(self.stack_pointer as usize)+1] as i32);
                self.stack_pointer -= 1;
                ticks = 9;
                execution_pointer_inc = 2;
            }
            instruction::STANDARD_OUTPUT_WRITE_INSTRUCTION => {
                let std_out = self.clone().standard_output;
                self.standard_output = std_out + ((data_1 as u8) as char).to_string().as_ref();
                ticks = 2;
                execution_pointer_inc = 2;
            }
            instruction::STANDARD_OUTPUT_CLEAR_INSTRUCTION => {
                self.standard_output = "".to_string();
                ticks = 1;
                execution_pointer_inc = 1;
            }
            instruction::LOAD_BYTE_INSTRUCTION => { result = Some(self.memory[data_2 as usize] as i32); ticks = 5; execution_pointer_inc = 3; },
            instruction::STORE_BYTE_INSTRUCTION => { self.memory[data_2 as usize] = data_1 as u8; ticks = 5; execution_pointer_inc = 3; },
            _=> result = None
        }


        // Update the execution pointer
        self.execution_pointer += execution_pointer_inc;

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