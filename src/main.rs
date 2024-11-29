use std::iter::Peekable;

use expr_eval::token::{Token, Tokenizer};
use expr_eval::r#type::{Result, ExprError};
use expr_eval::ASSOC_LEFT;


struct Expr<'a> {
    iter: Peekable<Tokenizer<'a>>,
}

impl<'a> Expr<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            iter: Tokenizer::new(src).peekable(),
        }
    }

    pub fn eval(&mut self) -> Result<i32> {
        let result = self.compute_expr(1)?;
        if self.iter.peek().is_some() {
            return Err(ExprError::Parse("Unexpected end of expr".into()));
        }
        Ok(result)
    }

    /// 计算单个 Token 或者子表达式
    fn compute_atom(&mut self) -> Result<i32> {
        // 如果是数字的话，直接返回
        match self.iter.peek() {
            Some(Token::Number(n)) => {
                let val = *n;
                self.iter.next();
                Ok(val)
            }
            // 如果是左括号的话，递归计算括号内的值
            Some(Token::LeftParen) => {
                self.iter.next();
                let result = self.compute_expr(1)?;
                match self.iter.next() {
                    Some(Token::RightParen) => (),
                    _ => return Err(ExprError::Parse("Unexpected character".into())),
                }
                Ok(result)
            }
            _ => {
                Err(ExprError::Parse(
                    "Expecting a number or left parenthesis".into(),
                ))
            }
        }
    }

    fn compute_expr(&mut self, min_prec: i32) -> Result<i32> {
        // 计算第一个 Token
        let mut atom_lhs = self.compute_atom()?;
        loop {
            // peek 方法返回的是一个不可变引用（Option<&T>），而不是值本身
            // 由于 Peekable 的内部实现，这个引用的生命周期与 self.iter 的不可变借用绑定
            // self.iter.peek() 的返回值是一个临时值，其不可变借用作用域由编译器推导，可以在合适时自动释放
            let cur_token = self.iter.peek();
            if cur_token.is_none() {
                break;
            }
            // Option<&Token> 的借用生命周期是临时的，匹配结束后借用就释放了
            // 如果匹配值的不可变引用，放入变量 token，会持续到 token 的作用域结束
            // 这会使得不可变借用比预期更长，导致和后续 self.iter.next() 的可变借用冲突
            // 这里解引用了引用并复制了 Token 值, 因为 Token 实现了 Copy trait
            // 原始借用 (&Token) 的生命周期结束，对 self.iter 的不可变借用也随之结束
            let token = *cur_token.unwrap();

            // 解引用（* 操作符）是将 引用 转换为其所指向的 值，但是否复制该值，取决于值的类型

            // 1. Token 一定是运算符
            // 2. Token 的优先级必须大于等于 min_prec
            if !token.is_operator() || token.precedence() < min_prec {
                break;
            }
            let mut next_prec = token.precedence();
            if token.assoc() == ASSOC_LEFT {
                next_prec += 1;
            }
            self.iter.next();

            // 递归计算右边的表达式
            let atom_rhs = self.compute_expr(next_prec)?;

            // 得到了两边的值，进行计算
            match token.compute(atom_lhs, atom_rhs) {
                Some(ans) => atom_lhs = ans,
                _ => return Err(ExprError::Parse("Unexpected expr".into())),
            }
        }
        Ok(atom_lhs)
    }
}
// 将一个算术表达式解析成连续的 Token
// 并通过 Iterator 返回，也可以通过 Peekable 接口获取


fn main() {
    let src = "92 + 5 + 5 * 27 - (92 - 12) / 4 + 26";
    let mut expr = Expr::new(src);
    let result = expr.eval();
    println!("expr = {}", src);

    println!("res = {:?}", result);
}
