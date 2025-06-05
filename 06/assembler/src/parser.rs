use anyhow::Result;

pub struct Parser {
    lines: std::vec::IntoIter<String>,
}

impl Parser {
    pub fn new(src: &str) -> Self {
        Parser {
            lines: src
                .lines()
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
                .into_iter(),
        }
    }

    fn read_next_line(&mut self) -> Option<String> {
        while let Some(line) = self.lines.next() {
            if !line.is_empty() {
                return Some(line);
            }
        }
        None
    }

    fn parse_c_instruction(&self, line: &str) -> Result<Instruction> {
        let mut chars = line.chars();

        let mut passed_string = String::new();

        let mut dest = None;
        let mut comp = None;
        let mut jump = None;

        while let Some(char) = chars.next() {
            match char {
                '=' => {
                    dest = Some(passed_string.clone());
                    passed_string.clear()
                }
                ';' => {
                    comp = Some(passed_string.clone());
                    passed_string.clear();
                    jump = Some(chars.collect());
                    break;
                }
                _ => passed_string.push(char),
            }
        }

        Ok(Instruction::CInst {
            dest,
            comp: comp.unwrap_or(passed_string),
            jump,
        })
    }

    fn parse(&mut self) -> Option<Instruction> {
        if let Some(line) = self.read_next_line() {
            let line = line.trim();
            if line.starts_with('@') {
                Some(Instruction::AInst(line[1..].to_string()))
            } else if line.starts_with('(') {
                Some(Instruction::LInst(line[1..line.len() - 1].to_string()))
            } else {
                Some(self.parse_c_instruction(line).unwrap())
            }
        } else {
            None
        }
    }
}

impl Iterator for Parser {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse()
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// A instruction
    AInst(String),
    /// Label instruction
    LInst(String),
    /// C instruction
    CInst {
        dest: Option<String>,
        comp: String,
        jump: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a_instruction() {
        let mut parser = Parser::new("@123");
        assert_eq!(parser.next(), Some(Instruction::AInst("123".to_string())));
    }

    #[test]
    fn test_parse_label_instruction() {
        let mut parser = Parser::new("(LOOP)");
        assert_eq!(parser.next(), Some(Instruction::LInst("LOOP".to_string())));
    }

    #[test]
    fn test_parse_multi_line() {
        let mut parser = Parser::new("@123\n(LABEL)\n@789");
        assert_eq!(parser.next(), Some(Instruction::AInst("123".to_string())));
        assert_eq!(parser.next(), Some(Instruction::LInst("LABEL".to_string())));
        assert_eq!(parser.next(), Some(Instruction::AInst("789".to_string())));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_parse_c_instruction() {
        let mut parser = Parser::new("D=M");
        assert_eq!(
            parser.next(),
            Some(Instruction::CInst {
                dest: Some("D".to_string()),
                comp: "M".to_string(),
                jump: None,
            })
        );
    }

    #[test]
    fn test_parse_c_instruction_with_jump_and_dest() {
        let mut parser = Parser::new("D=M;JGT");
        assert_eq!(
            parser.next(),
            Some(Instruction::CInst {
                dest: Some("D".to_string()),
                comp: "M".to_string(),
                jump: Some("JGT".to_string()),
            })
        );
    }

    #[test]
    fn test_parse_c_instruction_with_dest() {
        let mut parser = Parser::new("D=M");
        assert_eq!(
            parser.next(),
            Some(Instruction::CInst {
                dest: Some("D".to_string()),
                comp: "M".to_string(),
                jump: None,
            })
        );
    }

    #[test]
    fn test_parse_c_instruction_with_jump() {
        let mut parser = Parser::new("M;JGT");
        assert_eq!(
            parser.next(),
            Some(Instruction::CInst {
                dest: None,
                comp: "M".to_string(),
                jump: Some("JGT".to_string()),
            })
        );
    }

    #[test]
    fn test_ignore_whitespace() {
        let mut parser = Parser::new("    D=M");
        assert_eq!(
            parser.next(),
            Some(Instruction::CInst {
                dest: Some("D".to_string()),
                comp: "M".to_string(),
                jump: None,
            })
        );
    }
}
