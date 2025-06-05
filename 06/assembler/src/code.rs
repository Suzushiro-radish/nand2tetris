use crate::parser::Instruction;

pub fn code(instructions: Vec<Instruction>) -> Vec<String> {
    instructions.iter().map(instruction_to_code).collect()
}

fn instruction_to_code(instruction: &Instruction) -> String {
    match instruction {
        Instruction::AInst(number) => format!("{:016b}", number.parse::<u16>().unwrap()),
        Instruction::CInst { dest, comp, jump } => {
            inst_c_to_code(dest.clone(), comp.clone(), jump.clone())
        }
        _ => "".to_string(),
    }
}

fn inst_c_to_code(dest: Option<String>, comp: String, jump: Option<String>) -> String {
    format!(
        "111{}{}{}",
        comp_to_bin(comp),
        dest_to_bin(dest),
        jump_to_bin(jump)
    )
}

fn dest_to_bin(dest: Option<String>) -> String {
    match dest {
        Some(d) => match d.as_str() {
            "M" => "001".to_string(),
            "D" => "010".to_string(),
            "MD" => "011".to_string(),
            "A" => "100".to_string(),
            "AM" => "101".to_string(),
            "AD" => "110".to_string(),
            "AMD" => "111".to_string(),
            _ => "000".to_string(),
        },
        None => "000".to_string(),
    }
}

fn comp_to_bin(comp: String) -> String {
    match comp.as_str() {
        "0" => "0101010".to_string(),
        "1" => "0111111".to_string(),
        "-1" => "0111010".to_string(),

        "D" => "0001100".to_string(),
        "A" => "0110000".to_string(),
        "M" => "1110000".to_string(),

        "!D" => "0001101".to_string(),
        "!A" => "0110001".to_string(),
        "!M" => "1110001".to_string(),

        "-D" => "0001111".to_string(),
        "-A" => "0110011".to_string(),
        "-M" => "1110011".to_string(),

        "D+1" => "0011111".to_string(),
        "A+1" => "0110111".to_string(),
        "M+1" => "1110111".to_string(),

        "D-1" => "0001110".to_string(),
        "A-1" => "0110010".to_string(),
        "M-1" => "1110010".to_string(),

        "D+A" => "0000010".to_string(),
        "D+M" => "1000010".to_string(),

        "D-A" => "0010011".to_string(),
        "D-M" => "1010011".to_string(),

        "A-D" => "0000111".to_string(),
        "M-D" => "1000111".to_string(),
        // &
        "D&A" => "0000000".to_string(),
        "D&M" => "1000000".to_string(),
        // |
        "D|A" => "0010101".to_string(),
        "D|M" => "1010101".to_string(),
        _ => panic!("Invalid comp {}", comp),
    }
}

fn jump_to_bin(jmp: Option<String>) -> String {
    match jmp {
        Some(j) => match j.as_str() {
            "JGT" => "001".to_string(),
            "JEQ" => "010".to_string(),
            "JGE" => "011".to_string(),
            "JLT" => "100".to_string(),
            "JNE" => "101".to_string(),
            "JLE" => "110".to_string(),
            "JMP" => "111".to_string(),
            _ => "".to_string(),
        },
        None => "000".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_inst_0() {
        let instruction = Instruction::AInst("0".to_string());
        assert_eq!(instruction_to_code(&instruction), "0000000000000000");
    }

    #[test]
    fn test_a_inst_32767() {
        let instruction = Instruction::AInst("32767".to_string());
        assert_eq!(instruction_to_code(&instruction), "0111111111111111");
    }

    #[test]
    fn test_c_inst_add() {
        let instruction = Instruction::CInst {
            dest: None,
            comp: "D+1".to_string(),
            jump: None,
        };
        assert_eq!(instruction_to_code(&instruction), "1110011111000000");
    }

    #[test]
    fn test_c_inst_d_a() {
        let instruction = Instruction::CInst {
            dest: Some("D".to_string()),
            comp: "A".to_string(),
            jump: None,
        };
        assert_eq!(instruction_to_code(&instruction), "1110110000010000");
    }
}
