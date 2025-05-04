use crate::machine::machine::Machine;
use crate::instruction;
use std::process::Command;
use std::time::{Duration, Instant};
use colored::Colorize;
use crossterm::{
    execute,
    terminal::{Clear, ClearType, size},
    cursor::{MoveTo, Show, Hide},
    style::{Print, SetForegroundColor, Color},
    queue,
};
use std::io::{Write, stdout};
use std::thread::sleep;
use crossterm::style;

impl Machine {

    pub fn execute(&mut self, hertz: Option<u32>){
        let start_time = Instant::now();

        while self.flags & 0x4000_0000 < 1 && self.execution_pointer < 100 {
            // Visual
            // 1. Clear the screen
            Clear(ClearType::FromCursorDown);//println!("\x1b[2J");
            // 2. Delete scroll buffer
            //print!("\x1b[3J");
            stdout().flush().unwrap();
            // 3. Reset cursor position
            print!("\x1b[0;0H");
            stdout().flush().unwrap();

            // Execution
            let (instruction, ticks) = self.execute_line(hertz);

            // Visual Part 2

            // 1. Print the current instruction
            println!("Executing: {:?}                                                                                ", instruction);
            // 2. Print the registers
            self.print_registers();
            println!("{}", self.standard_output);
            // 3. Get terminal size & set cursor to the bottom
            let (_, height) = size().unwrap();
            print!("\x1b[{};0H", height);
            stdout().flush().unwrap();
            if let Some(hertz) = hertz {
                // Add 9 to ticks as the fetch cycle takes longer then expected
                let wait_time_s = 1. / (hertz as f32) * (ticks * 2 + 9) as f32;
                sleep(Duration::from_millis((wait_time_s * 1000.0) as u64));
            }

            let cursor_position_incremented = crossterm::cursor::position().unwrap().0 > 0;
            if cursor_position_incremented {


                if Self::await_command(self) {
                    return;
                }

                print!("\x1b[{};{}H", height, 0);
                stdout().flush().unwrap();
                stdout().flush().unwrap();
                Clear(ClearType::FromCursorUp);
            }
        }
        let duration = Instant::now().duration_since(start_time);
        println!("{}", format!("Execution finished after: {:?}", duration).green());

        loop {
            if Self::await_command(self) {return;}
        }

    }

    fn await_command(&mut self) -> bool /*Quit*/{
        let (width, height) = size().unwrap();
        print!("\x1b[{};{}H", height, 0);
        stdout().flush().unwrap();

        for _ in 0..width{
            print!(" ");
        }


        print!(":");
        stdout().flush().unwrap();
        print!("\x1b[{};{}H", height, 1);
        stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input) // Reads a line from stdin into `input`
            .expect("Failed to read line"); // Handle errors (replace with proper error handling)

        // Trim whitespace and newline characters
        let input = input.trim();
        println!("input: {}", input);

        let mut parts = input.split_whitespace();
        println!("parts: {:?}", parts);
        for part in parts.clone() {
            println!("part: {}", part);
        }
        let command = parts.nth(0).unwrap();

        // Process command
        match command {
            "quit" | "q" => true,
            ":" | "" | " " | "r" | "return" => false,
            "memget" | "mem-get" | "mg" => {
                if parts.clone().last().is_none(){
                    let warning = "Memory-Get needs at least 1 argument".yellow();
                    println!("{}", warning);
                    return false;
                }
                if let Some(index) = parts.last().unwrap().parse::<u32>().ok(){
                    if index >= self.memory.len() as u32{
                        let warning = "Index to high.".yellow();
                        println!("{}", warning);
                    }else{
                        println!("{}", self.memory[index as usize]);
                    }
                }else{
                    let warning = "Can't convert to positive 32 bit number.".yellow();
                    println!("{}", warning);
                }
                return false;
            }
            _ => { println!("Command not found"); return false; }
        };

        input == "q"
    }
    pub fn execute_line(&mut self, hertz: Option<u32>) -> (String/*Instruction name*/, u32 /*ticks*/){
        // Check halt bit & potentially avoid further execution
        if self.flags & 0x4000_0000 >= 1 { return ("Out of memory".to_string(), 1); }

        // Fetch
        let instruction = self.memory[self.execution_pointer as usize];
        let operand_1 = self.memory[(self.execution_pointer + 1) as usize];
        let operand_2 = self.memory[(self.execution_pointer + 2) as usize];

        let instruction_name = instruction::Instruction::name_of_instruction(instruction, operand_1, operand_2).unwrap_or("nothing.".yellow().to_string());


        let mut execution_pointer_inc: u32 = 0;


        // Get data from registers or pass the data from the operands
        let data_1 = if operand_1 >= 128 { self.data_of_register_by_value(operand_1 - 128) } else { operand_1 as i32 };
        let data_2 = if operand_2 >= 128 { self.data_of_register_by_value(operand_2 - 128) } else { operand_2 as i32 };

        // Create a result to update the register
        let mut result: Option<i32> = None;
        let mut ticks: u32 = 1;
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
                self.memory[self.stack_pointer as usize] = data_1 as u8;
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

        if operand_1 >= 128 && result != None{
            self.set_data_of_register(operand_1 - 128, result.unwrap());
        }

        (instruction_name, ticks + 9)
    }
}