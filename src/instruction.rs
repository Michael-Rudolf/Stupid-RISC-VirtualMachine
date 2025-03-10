#[derive(Copy)]
pub struct Instruction{
    task: u8,
    arg0: u8,
    arg1: u8,
}

// Task encoding after bits:
// Extension (1 = yes)
// If no extension:
// 1 means internal only
// 0 means external ops (RAM load, etc.)
// 010: ALU
// 011: Other internal (mov, etc)
// 000: Memory OP
// 001: Reserved for future applications
pub const STANDARD_OUTPUT_WRITE_INSTRUCTION: u8 = 0b0000_0001;
pub const STANDARD_OUTPUT_CLEAR_INSTRUCTION: u8 = 0b0000_0010;
pub const ADD_INSTRUCTION: u8 = 0b0100_0000;
pub const SUB_INSTRUCTION: u8 = 0b0100_0001;
pub const MUL_INSTRUCTION: u8 = 0b0100_0010;
pub const DIV_INSTRUCTION: u8 = 0b0100_0011;
pub const MOD_INSTRUCTION: u8 = 0b0100_0100;
pub const HALT_INSTRUCTION: u8 = 0b0110_0000;
pub const MOVE_INSTRUCTION: u8 = 0b0110_0001;
pub const JUMP_INSTRUCTION: u8 = 0b0110_0010;
pub const PUSH_BYTE_INSTRUCTION: u8 = 0b0110_0101;
pub const POP_BYTE_INSTRUCTION: u8 = 0b0110_1100;
// Jumps to arg2 if arg1 is 0
pub const JUMP_ZERO_INSTRUCTION: u8 = 0b0110_0011;
pub const LOAD_BYTE_INSTRUCTION: u8 = 0b0110_0100;
pub const STORE_BYTE_INSTRUCTION: u8 = 0b0111_0100;


#[allow(dead_code)]
pub const FLAGS_REGISTER: u8 = 12 + 128;
#[allow(dead_code)]
pub const EXEC_PTR_REGISTER: u8 = 15 + 128;
#[allow(dead_code)]
pub const EMPTY_ARGUMENT: u8 = 0;
impl Instruction{

    // Returns number of arguments and the name of the instruction
    pub fn name_of_instruction(task: u8, arg_1: u8, arg_2: u8) -> Option<String>{
        let arg_1 = if arg_1 >= 128 { format!("R{:02} ", arg_1 - 128) } else { format!("N{:02} ", arg_1) };
        let arg_1_text = arg_1.as_str();
        let arg_2 = if arg_2 >= 128 { format!("R{:02}", arg_2 - 128) } else { format!("N{:02}", arg_2) };
        let arg_2_text = arg_2.as_ref();
        match task{
            ADD_INSTRUCTION => Some("add ".to_string() + arg_1_text + arg_2_text),
            SUB_INSTRUCTION => Some("sub ".to_string() + arg_1_text + arg_2_text),
            MUL_INSTRUCTION => Some("mul ".to_string() + arg_1_text + arg_2_text),
            DIV_INSTRUCTION => Some("div ".to_string() + arg_1_text + arg_2_text),
            MOD_INSTRUCTION => Some("mod ".to_string() + arg_1_text + arg_2_text),
            HALT_INSTRUCTION => Some("halt ".to_string()),
            MOVE_INSTRUCTION => Some("mov ".to_string() + arg_1_text + arg_2_text),
            JUMP_INSTRUCTION => Some("jmp ".to_string() + arg_1_text),
            PUSH_BYTE_INSTRUCTION => Some("pushb ".to_string() + arg_1_text + arg_2_text),
            JUMP_ZERO_INSTRUCTION => Some("jmpz ".to_string() + arg_1_text + arg_2_text),
            LOAD_BYTE_INSTRUCTION => Some("ldb ".to_string() + arg_1_text + arg_2_text),
            STORE_BYTE_INSTRUCTION => Some("stb ".to_string() + arg_1_text + arg_2_text),
            POP_BYTE_INSTRUCTION => Some("popb ".to_string() + arg_1_text),
            STANDARD_OUTPUT_WRITE_INSTRUCTION => Some("sow ".to_string() + arg_1_text),
            STANDARD_OUTPUT_CLEAR_INSTRUCTION => Some("soc ".to_string()),
            _ => None
        }
    }
}

impl Clone for Instruction{
    fn clone(&self) -> Instruction{
        Instruction{task: self.task, arg0: self.arg0, arg1: self.arg1}
    }
}

