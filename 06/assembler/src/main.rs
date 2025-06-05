mod parser;

fn main() {
    let src = "M=1";
    let mut parser = parser::Parser::new(src);
    println!("{:?}", parser.next().expect("test"));
}
