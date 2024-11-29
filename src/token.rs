use std::fmt::Display;
use std::iter::Peekable;
use std::str::Chars;
use crate::{ASSOC_LEFT, ASSOC_RIGHT};

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(i32),
    Plus,       // 加
    Minus,      // 减
    Multiply,   // 乘
    Divide,     // 除
    Power,      // 幂
    LeftParen,  // 左括号
    RightParen, // 右括号
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Token::Number(n) => n.to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Multiply => "*".to_string(),
            Token::Divide => "/".to_string(),
            Token::Power => "^".to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
        };
        write!(f, "{}", char)
    }
}

impl Token {
    pub fn is_operator(&self) -> bool {
        match self {
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Power => true,
            _ => false,
        }
    }
    /// 获取运算符的优先级
    pub fn precedence(&self) -> i32 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Divide => 2,
            Token::Power => 3,
            _ => 0,
        }
    }
    /// 获取运算符的结合性
    pub fn assoc(&self) -> i32 {
        match self {
            Token::Power => ASSOC_RIGHT,
            _ => ASSOC_LEFT,
        }
    }
    pub fn compute(&self, l: i32, r: i32) -> Option<i32> {
        match self {
            Token::Plus => Some(l + r),
            Token::Minus => Some(l - r),
            Token::Multiply => Some(l * r),
            Token::Divide => Some(l / r),
            Token::Power => Some(l.pow(r as u32)),
            _ => None,
        }
    }
}


pub struct Tokenizer<'a> {
    tokens: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &'a str) -> Self {
        Self {
            tokens: expr.chars().peekable(),
        }
    }

    /// 消除左侧空白字符
    fn trim_left(&mut self) {
        // while let Some(ch) = self.tokens.peek() {
        //     if ch.is_whitespace() {
        //         self.tokens.next();
        //     } else {
        //         break;
        //     }
        // }
        self.tokens.next_if(|&ch| ch.is_whitespace());
    }

    /// 扫描数字
    fn scan_number(&mut self) -> Option<Token> {
        let mut num = String::new();
        // while let Some(&ch) = self.tokens.peek() {
        //     if ch.is_numeric() {
        //         num.push(ch);
        //         // 忘记下面这行代码了，导致无限迭代
        //         self.tokens.next();
        //     } else {
        //         break;
        //     }
        // }
        // 优化后，如下
        while let Some(ch) = self.tokens.next_if(|&ch| ch.is_numeric()) {
            num.push(ch);
        }
        match num.parse() {
            Ok(num) => Some(Token::Number(num)),
            _ => None,
        }
    }
    fn scan_operator(&mut self) -> Option<Token> {
        match self.tokens.next() {
            Some('+') => Some(Token::Plus),
            Some('-') => Some(Token::Minus),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Power),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            _ => None,
        }
    }
}

// 实现 Iterator 接口，使 Tokenizer 可以通过 for 循环遍历
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.trim_left();
        // 解析当前位置的 Token 类型
        match self.tokens.peek() {
            Some(c) if c.is_numeric() => self.scan_number(),
            Some(_) => self.scan_operator(),
            None => return None,
        }
    }
}