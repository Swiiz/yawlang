use crate::lexer::{self, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxNode {
    Program(Vec<Box<SyntaxNode>>), //Root node
    Function {
        name: String,
        parameters: Vec<String>,
        return_type: String,
        body: Box<SyntaxNode>,
    },

    Block(Vec<Box<SyntaxNode>>),
    
    ReturnStatement(Box<SyntaxNode>),

    Literals(lexer::Literal),
    Identifier(String),

    UnaryExpression {
        operator: lexer::Operator,
        operand: Box<SyntaxNode>,
    },
    BinaryExpression {
        operator: lexer::Operator,
        left: Box<SyntaxNode>,
        right: Box<SyntaxNode>,
    },

    Empty
}

impl SyntaxNode {
    pub fn generate_ast(tokens: Vec<lexer::Token>) -> SyntaxNode{
        let mut tokens = tokens;

        let mut nodes = Vec::new();
        while !tokens.is_empty() {
            nodes.push(Box::new(SyntaxNode::recursive_ast_gen(&mut tokens)));
        }
        SyntaxNode::Program(nodes)
    }

    fn recursive_ast_gen(tokens: &mut Vec<lexer::Token>) -> SyntaxNode {
        // Skip whitespace & comments
        if slice_token(tokens, 1)[0] == Token::Whitespace || slice_token(tokens, 1)[0] == Token::Comment
        || slice_token(tokens, 1)[0] == Token::Semicolon { // Need to rework semicolon
            tokens.remove(0);
            return SyntaxNode::recursive_ast_gen(tokens);
        //Function
        }else if let [Token::Identifier(return_type), Token::Whitespace, Token::Identifier(fun_name), Token::OpenParenthesis, Token::CloseParenthesis] = slice_token(&tokens.clone(), 5) {
            tokens.drain(..5);
            return SyntaxNode::Function {
                name: fun_name.clone(),
                parameters: vec![],
                return_type: return_type.clone(),
                body: Box::new(SyntaxNode::recursive_ast_gen(tokens)),
            }
        //Block
        }else if let [Token::OpenBrace] = slice_token(&tokens.clone(), 1) {
            tokens.drain(..1);
            let mut nodes = Vec::new();
            while &tokens.clone()[0] != &Token::CloseBrace {
                nodes.push(Box::new(SyntaxNode::recursive_ast_gen(tokens)));
            }
            tokens.drain(..1);
            return SyntaxNode::Block(nodes)
        //Return
        }else if let [Token::Keyword(lexer::Keyword::Return)] = slice_token(&tokens.clone(), 1) {
            tokens.drain(..1);
            return SyntaxNode::ReturnStatement(Box::new(SyntaxNode::recursive_ast_gen(tokens)));
        //Literals
        }else if let [Token::Literal(literal)] = slice_token(&tokens.clone(), 1) {
            tokens.drain(..1);
            return SyntaxNode::Literals(literal.clone());
        }else {
            return SyntaxNode::Empty;
        }
    }
}

fn slice_token(tokens: &Vec<Token>, end: usize) -> &[Token] {
    if tokens.len() >= end {
        &(*tokens)[..end]
    }else {
        &[]
    }
}