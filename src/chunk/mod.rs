use op_code::OpCode;
use std::fmt::Debug;

pub mod op_code;
pub mod value;

pub struct Chunk {
    code: Vec<OpCode>,
    lines: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            lines: vec![],
        }
    }

    pub fn write(&mut self, byte: OpCode, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let manage_code = |(idx, code): (usize, &OpCode)| {
            let line = if idx > 0 && self.lines[idx] == self.lines[idx - 1] {
                "  |".to_string()
            } else {
                format!("{}", self.lines[idx])
            };

            match code {
                OpCode::OpConstant(value) => {
                    format!("{:04}\t{} {:?}\t\t{}\n", idx, line, code, value)
                }
                OpCode::OpReturn => format!("{:04}\t{} {:?}\n", idx, line, code),
            }
        };

        let output: Vec<String> = self.code.iter().enumerate().map(manage_code).collect();
        write!(f, "{}", output.join(""))
    }
}

// impl Debug for Chunk {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut output = String::new();

//         let mut offset = 0;

//         while offset < self.code.len() {
//             output.push_str(format!("{:04}\t", offset).as_str());

//             let op_code = self.code[offset];
//             let op_code = OpCode::from(&op_code);

//             if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
//                 output.push_str("  | ")
//             } else {
//                 output.push_str(format!("{} ", self.lines[offset]).as_str());
//             }

//             offset = match op_code {
//                 OpCode::OpConstant => {
//                     let constant_index = self.code.get(offset).unwrap();

//                     let constant = self.constants.get(*constant_index as usize).unwrap();

//                     let constant_output = format!(
//                         "{:?}\t{} '{}'\n",
//                         OpCode::from(op_code),
//                         constant_index,
//                         constant
//                     );
//                     output.push_str(constant_output.as_str());
//                     offset + 2
//                 }
//                 OpCode::OpReturn => {
//                     output
//                         .push_str(format!("{:?}\n", OpCode::from(op_code)).as_str());
//                     offset + 1
//                 }
//             }
//         }

//         write!(f, "{}", output)
//     }
// }
