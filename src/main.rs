use std::{fs::File, io::Read};

use yawlang::{lexer, parser, asmgen};

fn main() {
    let mut filepath = std::env::args().nth(1).expect("No file given");
    if !filepath.ends_with(".yaw") {
        filepath.push_str(".yaw");
    }

    let mut file = File::open(filepath.clone()).expect("Invalid file path.");
    let mut code = String::new();
    file.read_to_string(&mut code).expect("Failed to read the provided file.");

    let tokens = lexer::Token::extract_tokens(code.as_str());
    println!("{:?}", tokens);

    let ast = parser::SyntaxNode::generate_ast(tokens);
    println!("{:?}", ast);

    let mut filename = filepath.split("/").last().unwrap().to_string();
    filename.push_str(".s");
    let target_file = asmgen::AssemblerTargetFile::from_ast(filename, ast);
    println!("{:?}", target_file);

    target_file.write_to_build_folder();
}
