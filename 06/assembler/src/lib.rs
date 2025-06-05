mod code;
mod parser;

use anyhow::Result;

/// Assembler for the Hack assembly language.
/// ```
/// use assembler::Assembler;
///
/// let src = String::from("@123\nD=D+A");
///
/// let assembler = Assembler::new(src);
/// let result = assembler.assemble().unwrap();
///
/// assert_eq!(result, "0000000001111011\n1110000010010000");
/// ```
pub struct Assembler {
    parser: parser::Parser,
}

impl Assembler {
    pub fn new(src: String) -> Self {
        Assembler {
            parser: parser::Parser::new(&src),
        }
    }

    pub fn assemble(self) -> Result<String> {
        let instructions = self.parser.collect();
        let code = code::code(instructions).join("\n");
        Ok(code)
    }
}
