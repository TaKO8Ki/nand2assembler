use std::collections::HashMap;

lazy_static! {
    static ref COMP_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("0", "0101010");
        m.insert("1", "0111111");
        m.insert("-1", "0111010");
        m.insert("D", "0001100");
        m.insert("A", "0110000");
        m.insert("!D", "0001101");
        m.insert("!A", "0110011");
        m.insert("-D", "0001111");
        m.insert("-A", "0110011");
        m.insert("D+1", "0011111");
        m.insert("A+1", "0110111");
        m.insert("D-1", "0001110");
        m.insert("A-1", "0110010");
        m.insert("D+A", "0000010");
        m.insert("D-A", "0010011");
        m.insert("A-D", "0000111");
        m.insert("M", "1110000");
        m.insert("!M", "1110001");
        m.insert("-M", "1110011");
        m.insert("M+1", "1110111");
        m.insert("M-1", "1110010");
        m.insert("D+M", "1000010");
        m.insert("D-M", "1010011");
        m.insert("M-D", "1000111");
        m.insert("D&M", "1000000");
        m.insert("D|M", "1010101");
        m
    };
    static ref DEST_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("null", "000");
        m.insert("M", "001");
        m.insert("D", "010");
        m.insert("MD", "011");
        m.insert("A", "100");
        m.insert("AM", "101");
        m.insert("AD", "110");
        m.insert("AMD", "111");
        m
    };
    static ref JUMP_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("null", "000");
        m.insert("JGT", "001");
        m.insert("JEQ", "010");
        m.insert("JGE", "011");
        m.insert("JLT", "100");
        m.insert("JNE", "101");
        m.insert("JLE", "110");
        m.insert("JMP", "111");
        m
    };
}

pub fn dest(mnemonic: Option<String>) -> String {
    let dest_mnemonic = match mnemonic {
        Some(mn) => mn,
        None => "null".to_string(),
    };
    let key: &str = &dest_mnemonic;
    DEST_MAP.get(key).unwrap().to_string()
}

pub fn comp(mnemonic: Option<String>) -> String {
    let comp_mnemonic = match mnemonic {
        Some(mn) => mn,
        None => "".to_string(),
    };
    let key: &str = &comp_mnemonic;
    match COMP_MAP.get(key) {
        Some(num) => return num.to_string(),
        None => return "".to_string(),
    }
}

pub fn jump(mnemonic: Option<String>) -> String {
    let jump_mnemonic = match mnemonic {
        Some(mn) => mn,
        None => "null".to_string(),
    };
    let key: &str = &jump_mnemonic;
    JUMP_MAP.get(key).unwrap().to_string()
}
