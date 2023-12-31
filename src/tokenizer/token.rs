#![allow(dead_code)]
/// 定义了Token的表达.
#[derive(Debug, Clone)]
pub enum Token {
    ParL,
    ParR,
    Quote,
    QuasiQuote,
    Unquote,
    Dot,
    Boolean(bool),
    Numeric(f64),
    String(String),
    Identifier(String),
}
impl ToString for Token {
    /// 将Token使用字符串字面量表示出来. 
    /// 仅用于调试
    fn to_string(&self) -> String {
        match self {
            Token::Boolean(b) => format!("(BOOLEAN_LITERAL {} )", b),
            Token::Numeric(f) => format!("NUMERIC_LITERAL {})", f),
            Token::String(s) => format!("STRING_LITERAL {:?})", s),
            Token::Identifier(s) => format!("IDENTIFIER {})", s),
            Token::ParL => format!("LEFT_PARENTHESIS"),
            Token::ParR => format!("RIGHT_PARENTHESIS"),
            Token::Quote => format!("QUOTE"),
            Token::QuasiQuote => format!("QUASIQUOTE"),
            Token::Unquote => format!("UNQUOTE"),
            Token::Dot => format!("DOT"),
        }
    }
}