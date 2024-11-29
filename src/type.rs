use std::fmt::Display;

// 自定义错误类型
#[derive(Debug)]
pub enum ExprError {
    Parse(String),
}

impl Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprError::Parse(err) => write!(f, "{}", err),
        }
    }
}

// 自定义 Result 类型
pub type Result<T> = std::result::Result<T, ExprError>;