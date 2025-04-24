use std::collections::HashMap;
use std::f32::consts::E;
use std::rc::Rc;
use std::thread::sleep;
use crate::lex::{Parser, Token, TokenType};
use crate::error::error::CustomError;

#[derive(Debug)]
struct NumberAst {
    val: i64,
}
#[derive(Debug)]
struct BinaryAst {
    op: String,
    left: Rc<AstNode>,
    right: Rc<AstNode>,
}

#[derive(Debug)]
struct FunCallerAST {
    name: String,
    arg: Vec<Rc<AstNode>>,
}

#[derive(Debug)]
pub enum AstNode {
    Number(NumberAst),
    Binary(BinaryAst),
    FunCaller(FunCallerAST),
}

pub struct Ast<'a> {
    tokens: &'a [Token],       // 使用切片引用，避免所有权转移
    curr_tok: Option<&'a Token>, // 当前 token，使用引用并支持空值
    curr_idx: usize,           // 当前索引
    depth: i8,                 // 深度
    priority_map: HashMap<String, i8>
}

impl<'a> Ast<'a> {
    /// 创建新的 Ast 实例
    pub fn new_ast(tokens: &'a [Token]) -> Result<Self, CustomError> {
        if tokens.is_empty() {
            return Err(CustomError::InvalidOffset);
        }
        Ok(Self {
            tokens,
            curr_tok: tokens.get(0), // 获取第一个 token 的引用
            curr_idx: 0,
            depth: 0,
            priority_map: HashMap::from([
                ("+".to_string(), 1),
                ("-".to_string(), 1),
                ("*".to_string(), 40),
                ("/".to_string(), 40),
                ("%".to_string(), 40),
            ]),
        })
    }

    /// 解析表达式
    pub fn parse_expression(&mut self) -> Result<Rc<AstNode>, CustomError> {
        self.depth+=1;
        let left = self.parse_primary();
        let right = self.parse_op(0,left?);
        //println!("{:?}", right);
        right
    }

    pub fn parse_op(&mut self,mut priority: i8,mut left:  Rc<AstNode>)-> Result<Rc<AstNode>,CustomError>{
        loop{
            println!("---left--- {:?}", left);
        let op = self.curr_tok;
        match op {
            Some(token) => {
                match token.tok_type {
                    TokenType::OPERATOR => {
                        let cur_priority = self.priority_map.get(&token.tok).ok_or(CustomError::InvalidSyntax)?;
                        if cur_priority < &priority {
                            return Ok(left);
                        }
                        self.curr_idx += 1;
                        self.curr_tok = self.tokens.get(self.curr_idx);
                        let mut right  = self.parse_primary()?;

                        let cur_priority = self.priority_map.get(&token.tok).ok_or(CustomError::InvalidSyntax)?;
                        let next_priority = self.priority_map.get(&self.curr_tok.ok_or(CustomError::InvalidSyntax)?.tok).ok_or(CustomError::InvalidSyntax)?;

                        println!("1 cur:{:?}",cur_priority);
                        println!("2 next:{:?}", next_priority);
                        println!("right {:?}", right);
                        if cur_priority < next_priority {
                            right = self.parse_op(cur_priority+1,right)?;
                        }
                        left = Rc::from(AstNode::Binary(
                                BinaryAst {
                                    op: token.tok.clone(),
                                    left: left,
                                    right: right,
                                }
                            ))

                    },
                    _ => {
                        return Err(CustomError::InvalidSyntax)
                    }
                    _=>{
                      return   Err(CustomError::InvalidSyntax)
                    }
                }
            },
            None => {
              return   Err(CustomError::InvalidSyntax)
            }
        }
        }
    }


    fn parse_primary(&mut self)-> Result<Rc<AstNode>,CustomError>{
        match self.curr_tok {
            Some(token) => {
                match token.tok_type {
                    TokenType::LITERAL => {
                        let num = NumberAst {
                            val: token.tok.parse::<i64>().unwrap(),
                        };
                        self.curr_idx += 1;
                        self.curr_tok = self.tokens.get(self.curr_idx);
                        Ok(Rc::new(AstNode::Number(num)))
                    },
                    TokenType::IDENTIFIER => {
                        Err(CustomError::InvalidSyntax)
                    },
                    TokenType::COMMA => {
                        Err(CustomError::InvalidSyntax)
                    },
                    _ => {
                        Err(CustomError::InvalidSyntax)
                    }
                }
            },
            None => {
                Err(CustomError::InvalidSyntax)
            }
        }
    }

}
