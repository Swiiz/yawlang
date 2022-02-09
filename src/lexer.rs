#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Whitespace,
    Comment,

    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Semicolon,

    Literal(Literal),
    
    Keyword(Keyword),
    Operator(Operator),
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Let,
    If,
    Else,

    Return,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Integer(i64),   
    Float(f64),
    Double(f64),
    Character(char),
    String(String),
    Boolean(bool),
}

use lazy_static::lazy_static;
use regex::Regex;
lazy_static!{
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"^\s+").unwrap();
    static ref COMMENT_REGEX: Regex = Regex::new(r"^//[^\n]*").unwrap();
    
    static ref OPEN_PARENTHESIS_REGEX: Regex = Regex::new(r"^\(").unwrap();
    static ref CLOSE_PARENTHESIS_REGEX: Regex = Regex::new(r"^\)").unwrap();
    static ref OPEN_BRACKET_REGEX: Regex = Regex::new(r"^\[").unwrap();
    static ref CLOSE_BRACKET_REGEX: Regex = Regex::new(r"^\]").unwrap();
    static ref OPEN_BRACE_REGEX: Regex = Regex::new(r"^\{").unwrap();
    static ref CLOSE_BRACE_REGEX: Regex = Regex::new(r"^\}").unwrap();
    static ref SEMICOLON_REGEX: Regex = Regex::new(r"^;").unwrap();

    static ref INTEGER_LITERAL_REGEX: Regex = Regex::new(r"^\d+").unwrap();
    static ref FLOAT_LITERAL_REGEX: Regex = Regex::new(r"^\d+\.\d+f").unwrap();
    static ref DOUBLE_LITERAL_REGEX: Regex = Regex::new(r"^\d+\.\d+").unwrap();
    static ref CHARACTER_LITERAL_REGEX: Regex = Regex::new(r"^'[^']'").unwrap();
    static ref STRING_LITERAL_REGEX: Regex = Regex::new(r#"^"[^"]*""#).unwrap();
    static ref BOOLEAN_LITERAL_REGEX: Regex = Regex::new(r"^(true|false)").unwrap();

    static ref KEYWORD_LET_REGEX: Regex = Regex::new(r"^let").unwrap();
    static ref KEYWORD_IF_REGEX: Regex = Regex::new(r"^if").unwrap();
    static ref KEYWORD_ELSE_REGEX: Regex = Regex::new(r"^else").unwrap();

    static ref KEYWORD_RETURN_REGEX: Regex = Regex::new(r"^return").unwrap();

    static ref OPERATOR_PLUS_REGEX: Regex = Regex::new(r"^\+").unwrap();
    static ref OPERATOR_MINUS_REGEX: Regex = Regex::new(r"^\-").unwrap();
    static ref OPERATOR_MULTIPLY_REGEX: Regex = Regex::new(r"^\*").unwrap();
    static ref OPERATOR_DIVIDE_REGEX: Regex = Regex::new(r"^/").unwrap();
    static ref OPERATOR_EQUAL_REGEX: Regex = Regex::new(r"^==").unwrap();
    static ref OPERATOR_NOT_EQUAL_REGEX: Regex = Regex::new(r"^!=").unwrap();
    static ref OPERATOR_LESS_THAN_REGEX: Regex = Regex::new(r"^<").unwrap();
    static ref OPERATOR_GREATER_THAN_REGEX: Regex = Regex::new(r"^>").unwrap();
    static ref OPERATOR_LESS_THAN_OR_EQUAL_REGEX: Regex = Regex::new(r"^<=").unwrap();
    static ref OPERATOR_GREATER_THAN_OR_EQUAL_REGEX: Regex = Regex::new(r"^>=").unwrap();

    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
}

impl Token {
    pub fn extract_tokens(code: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut code = code;
        while !code.is_empty() {
            match Token::find_next_token(&mut code) {
                Ok(token) => tokens.push(token),
                Err(reason) => panic!("An exception occured during the token extraction:\n {}", reason),
            }
        }
        tokens
    }

    fn find_next_token(input: &mut &str) -> Result<Token, String> {
        if WHITESPACE_REGEX.is_match(input) {
            *input = &input[WHITESPACE_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Whitespace);
        }else if COMMENT_REGEX.is_match(input) {
            *input = &input[COMMENT_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Comment);
        }else if OPEN_PARENTHESIS_REGEX.is_match(input) {
            *input = &input[OPEN_PARENTHESIS_REGEX.find(input).unwrap().end()..];
            return Ok(Token::OpenParenthesis);
        }else if CLOSE_PARENTHESIS_REGEX.is_match(input) {
            *input = &input[CLOSE_PARENTHESIS_REGEX.find(input).unwrap().end()..];
            return Ok(Token::CloseParenthesis);
        }else if OPEN_BRACKET_REGEX.is_match(input) {
            *input = &input[OPEN_BRACKET_REGEX.find(input).unwrap().end()..];
            return Ok(Token::OpenBracket);
        }else if CLOSE_BRACKET_REGEX.is_match(input) {
            *input = &input[CLOSE_BRACKET_REGEX.find(input).unwrap().end()..];
            return Ok(Token::CloseBracket);
        }else if OPEN_BRACE_REGEX.is_match(input) {
            *input = &input[OPEN_BRACE_REGEX.find(input).unwrap().end()..];
            return Ok(Token::OpenBrace);
        }else if CLOSE_BRACE_REGEX.is_match(input) {
            *input = &input[CLOSE_BRACE_REGEX.find(input).unwrap().end()..];
            return Ok(Token::CloseBrace);
        }else if SEMICOLON_REGEX.is_match(input) {
            *input = &input[SEMICOLON_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Semicolon);
        }else if INTEGER_LITERAL_REGEX.is_match(input) {
            let integer_literal = &input[INTEGER_LITERAL_REGEX.find(input).unwrap().start()..INTEGER_LITERAL_REGEX.find(input).unwrap().end()];
            *input = &input[INTEGER_LITERAL_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Literal(Literal::Integer(integer_literal.parse::<i64>().unwrap())));
        }else if FLOAT_LITERAL_REGEX.is_match(input) {
            let float_literal = &input[FLOAT_LITERAL_REGEX.find(input).unwrap().start()..FLOAT_LITERAL_REGEX.find(input).unwrap().end()];
            *input = &input[FLOAT_LITERAL_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Literal(Literal::Float(float_literal.parse::<f64>().unwrap())));
        }else if DOUBLE_LITERAL_REGEX.is_match(input) {
            let double_literal = &input[DOUBLE_LITERAL_REGEX.find(input).unwrap().start()..DOUBLE_LITERAL_REGEX.find(input).unwrap().end()];
            *input = &input[DOUBLE_LITERAL_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Literal(Literal::Double(double_literal.parse::<f64>().unwrap())));
        }else if CHARACTER_LITERAL_REGEX.is_match(input) {
            let character_literal = &input[CHARACTER_LITERAL_REGEX.find(input).unwrap().start()..CHARACTER_LITERAL_REGEX.find(input).unwrap().end()];
            *input = &input[CHARACTER_LITERAL_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Literal(Literal::Character(character_literal.chars().nth(1).unwrap())));
        }else if STRING_LITERAL_REGEX.is_match(input) {
            let string_literal = &input[STRING_LITERAL_REGEX.find(input).unwrap().start()..STRING_LITERAL_REGEX.find(input).unwrap().end()];
            *input = &input[STRING_LITERAL_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Literal(Literal::String(string_literal.to_string())));
        }else if BOOLEAN_LITERAL_REGEX.is_match(input) {
            let boolean_literal = &input[BOOLEAN_LITERAL_REGEX.find(input).unwrap().start()..BOOLEAN_LITERAL_REGEX.find(input).unwrap().end()];
            *input = &input[BOOLEAN_LITERAL_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Literal(Literal::Boolean(boolean_literal.parse::<bool>().unwrap())));
        }else if KEYWORD_LET_REGEX.is_match(input) {
            *input = &input[KEYWORD_LET_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Keyword(Keyword::Let));
        }else if KEYWORD_IF_REGEX.is_match(input) {
            *input = &input[KEYWORD_IF_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Keyword(Keyword::If));
        }else if KEYWORD_ELSE_REGEX.is_match(input) {
            *input = &input[KEYWORD_ELSE_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Keyword(Keyword::Else));
        }else if KEYWORD_RETURN_REGEX.is_match(input) {
            *input = &input[KEYWORD_RETURN_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Keyword(Keyword::Return));
        }else if OPERATOR_PLUS_REGEX.is_match(input) {
            *input = &input[OPERATOR_PLUS_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Operator(Operator::Plus));
        }else if OPERATOR_MINUS_REGEX.is_match(input) {
            *input = &input[OPERATOR_MINUS_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Operator(Operator::Minus));
        }else if OPERATOR_MULTIPLY_REGEX.is_match(input) {
            *input = &input[OPERATOR_MULTIPLY_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Operator(Operator::Multiply));
        }else if OPERATOR_DIVIDE_REGEX.is_match(input) {
            *input = &input[OPERATOR_DIVIDE_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Operator(Operator::Divide));
        }else if OPERATOR_EQUAL_REGEX.is_match(input) {
            *input = &input[OPERATOR_EQUAL_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Operator(Operator::Equal));
        }else if OPERATOR_NOT_EQUAL_REGEX.is_match(input) {
            *input = &input[OPERATOR_NOT_EQUAL_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Operator(Operator::NotEqual));
        }else if OPERATOR_LESS_THAN_REGEX.is_match(input) {
            *input = &input[OPERATOR_LESS_THAN_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Operator(Operator::LessThan));
        }else if OPERATOR_LESS_THAN_OR_EQUAL_REGEX.is_match(input) {
            *input = &input[OPERATOR_LESS_THAN_OR_EQUAL_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Operator(Operator::LessThanOrEqual));
        }else if OPERATOR_GREATER_THAN_REGEX.is_match(input) {
            *input = &input[OPERATOR_GREATER_THAN_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Operator(Operator::GreaterThan));
        }else if OPERATOR_GREATER_THAN_OR_EQUAL_REGEX.is_match(input) {
            *input = &input[OPERATOR_GREATER_THAN_OR_EQUAL_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Operator(Operator::GreaterThanOrEqual));
        }else if IDENTIFIER_REGEX.is_match(input) {
            let identifier = &input[IDENTIFIER_REGEX.find(input).unwrap().start()..IDENTIFIER_REGEX.find(input).unwrap().end()];
            *input = &input[IDENTIFIER_REGEX.find(input).unwrap().end()..];
            return Ok(Token::Identifier(identifier.to_string()));
        }else {
            return Err(format!("Invalid token: {}", input));
        }
    }
}