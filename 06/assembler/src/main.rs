extern crate assembler;

use assembler::Assembler;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_file(filepath: &str) -> String {
    let mut f = File::open(filepath).expect("file not found");
    let mut src = String::new();
    f.read_to_string(&mut src).expect("failed to read file");
    src
}

fn write_file(filepath: &str, content: String) {
    let output_path = Path::new(filepath).with_extension("hack");
    let mut out = File::create(&output_path).expect("failed to create output file");
    out.write_all(content.as_bytes())
        .expect("failed to write to output file");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let src = read_file(filepath);

    let assembler = Assembler::new(src);
    let code = assembler.assemble().expect("failed to assemble");

    write_file(filepath, code);
}
