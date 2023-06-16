use op_code::OpCode;
use std::fmt::Debug;

pub mod op_code;

type Value = f64;

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, value: Value, line: u32) {
        self.constants.push(value);
        self.write(OpCode::OpConstant as u8, line);
        self.write((self.constants.len() - 1) as u8, line);
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        let mut offset = 0;

        while offset < self.code.len() {
            output.push_str(format!("{:04}\t", offset).as_str());

            let op_code = self.code[offset];
            let op_code = OpCode::from(&op_code);

            if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
                output.push_str("  | ")
            } else {
                output.push_str(format!("{} ", self.lines[offset]).as_str());
            }

            offset = match op_code {
                OpCode::OpConstant => {
                    let constant_index = self.code.get(offset).unwrap();

                    let constant = self.constants.get(*constant_index as usize).unwrap();

                    let constant_output = format!(
                        "{:?}\t{} '{}'\n",
                        OpCode::from(op_code),
                        constant_index,
                        constant
                    );
                    output.push_str(constant_output.as_str());
                    offset + 1
                }
                OpCode::OpReturn => {
                    output
                        .push_str(format!("{:?}\n", OpCode::from(op_code)).as_str());
                    offset + 1
                }
            }
        }

        write!(f, "{}", output)
    }
}

// impl Debug for Chunk {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut output = String::new();

//         let mut code = self.code.iter();

//         while let Some(offset) = code.next() {
//             // let line_number = if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
//             //     "  | ".to_string()
//             // } else {
//             //     format!("{} ", self.lines[offset])
//             // };

//             code.skip(2);
//         }

        // while code.is_some() {
        //     let (offset, code) = code.unwrap();

        // }

        // while offset < self.code.len() {
        //     output.push_str(format!("{:04}\t", offset).as_str());

        //     let op_code = self.code[offset];
        //     let op_code = OpCode::from(&op_code);

        //     if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
        //         output.push_str("  | ")
        //     } else {
        //         output.push_str(format!("{} ", self.lines[offset]).as_str());
        //     }

        //     offset = match op_code {
        //         OpCode::OpConstant => {
        //             let constant_index = self.code.get(offset).unwrap();

        //             let constant = self.constants.get(*constant_index as usize).unwrap();

        //             let constant_output = format!(
        //                 "{:?}\t{} '{}'\n",
        //                 OpCode::from(op_code),
        //                 constant_index,
        //                 constant
        //             );
        //             output.push_str(constant_output.as_str());
        //             offset + 2
        //         }
        //         OpCode::OpReturn => {
        //             output
        //                 .push_str(format!("{:?}\n", OpCode::from(op_code)).as_str());
        //             offset + 1
        //         }
        //     }
        // }

//         write!(f, "{}", output)
//     }
// }
