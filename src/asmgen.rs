use std::{io::Write, env};

use crate::{parser, lexer};

#[derive(Debug)]
pub struct AssemblerTargetFile {
    filename: String,
    content: String,
}

impl AssemblerTargetFile {
    pub fn write_to_build_folder(&self) {
        let mut filepath = String::new();
        filepath.push_str(env::current_dir().unwrap().to_str().unwrap());
        filepath.push_str("/build/");
        filepath.push_str(&self.filename);
        let mut file = std::fs::File::create(filepath).expect("Failed to create file.");
        file.write_all(self.content.as_bytes()).expect("Failed to write to file.");
    }

    pub fn from_ast(filename: String, ast: parser::SyntaxNode) -> AssemblerTargetFile {
        if let parser::SyntaxNode::Program(nodes) = ast {
            let mut content = String::new();
            for node in nodes {
                content.push_str(&AssemblerTargetFile::ast_to_assembler(&node));
            }
            AssemblerTargetFile {
                filename,
                content,
            }
        }else {
            panic!("The abstract syntax tree is not a program.");
        }
    }

    fn ast_to_assembler(node: &parser::SyntaxNode) -> String {
        if let parser::SyntaxNode::Program(nodes) = node {
            let mut code = String::new();
            for node in nodes {
                code.push_str(&AssemblerTargetFile::ast_to_assembler(node.as_ref()));
            }
            code
        }else if let parser::SyntaxNode::Function {
            name,
            parameters,
            return_type,
            body,
        } = node {
            let mut code = String::new();
            code.push_str(".globl ");
            code.push_str(name);
            code.push_str("\n");
            code.push_str(name);
            code.push_str(":\n");
            code.push_str(&AssemblerTargetFile::ast_to_assembler(body.as_ref()));
            code
        }else if let parser::SyntaxNode::ReturnStatement(node) = node {
            let mut code = String::new();
            if let parser::SyntaxNode::Literals(lexer::Literal::Integer(value)) = node.as_ref() {
                code.push_str("movl $");
                code.push_str(&value.to_string());
                code.push_str(", %eax\n");
            }
            //TODO: handle more than just integer litteral
            code.push_str("ret\n");
            code
        }else if let parser::SyntaxNode::Block(nodes) = node {
            let mut code = String::new();
            for node in nodes {
                code.push_str(&AssemblerTargetFile::ast_to_assembler(node.as_ref()));
            }
            code
        }else {
            println!("WARNING: Unknown Abstract Syntax Node found: {:?}", node);
            String::new()
        }
    }
}