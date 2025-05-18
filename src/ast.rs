use std::collections::HashMap;
use std::f32::consts::E;
use std::fmt::Alignment::Left;
use std::ptr::hash;
use std::rc::Rc;
use std::thread::sleep;
use crate::lex::{Parser, Token, TokenType};
use crate::error::error::CustomError;

#[derive(Debug)]
pub struct NumberAst {
    pub val: f64,
}
#[derive(Debug)]
pub struct BinaryAst {
   pub op: String,
   pub left: Rc<AstNode>,
   pub right: Rc<AstNode>,
}

#[derive(Debug)]
pub struct FunCallerAST {
    pub name: String,
    pub arg: Vec<Rc<AstNode>>,
}



#[derive(Debug)]
pub struct Func {
   pub   argc: i8,
   pub func:  fn(args: &[f64]) -> f64
}


fn func_min(args: &[f64])->f64{
    let mut min = args[0];
    args.iter().for_each(|x: &f64| {
        if *x < min {
            min = *x;
        }
    });
    min
}

fn func_max(args: &[f64])-> f64{
    let mut max = args[0];
    args.iter().for_each(|x: &f64| {
        if *x > max {
            max = *x;
        }
    });
    max
}

fn func_abs(args: &[f64])-> f64{
    args[0].abs()
}
fn func_sqrt(args: &[f64])-> f64{
    args[0].sqrt()
}
fn func_exp(args: &[f64])-> f64{
    args[0].exp()
}
fn func_log(args: &[f64])-> f64{
    args[0].ln()
}
fn func_sin(args: &[f64])-> f64{
    args[0].sin()
}
fn func_cos(args: &[f64])-> f64{
    args[0].cos()
}
fn func_tan(args: &[f64])-> f64{
    args[0].tan()
}
fn func_asin(args: &[f64])-> f64{
    args[0].asin()
}

fn func_acos(args: &[f64])-> f64{
    args[0].acos()
}
fn func_atan(args: &[f64])-> f64{
    args[0].atan()
}
fn func_atan2(args: &[f64])-> f64{
    args[0].atan2(args[1])
}
fn func_pow(args: &[f64])-> f64{
    args[0].powf(args[1])
}
fn func_log10(args: &[f64])-> f64{
    args[0].log10()
}

fn func_floor(args: &[f64])-> f64{
    args[0].floor()
}
fn func_ceil(args: &[f64])-> f64{
    args[0].ceil()
}



lazy_static::lazy_static! {
    static ref HASH_MAP: HashMap<&'static str, Func> = HashMap::from([
        ("min", Func { argc: 1, func: func_min }),
        ("max", Func { argc: 1, func: func_max }),
        ("abs", Func { argc: 1, func: func_abs }),
        ("floor", Func { argc: 1, func: func_floor }),
        ("ceil", Func { argc: 1, func: func_ceil }),
        ("sqrt", Func { argc: 1, func: func_sqrt }),
        ("exp", Func { argc: 1, func: func_exp }),
        ("log", Func { argc: 1, func: func_log }),
        ("log10", Func { argc: 1, func: func_log10 }),
        ("sin", Func { argc: 1, func: func_sin }),
        ("cos", Func { argc: 1, func: func_cos }),
        ("tan", Func { argc: 1, func: func_tan }),
        ("asin", Func { argc: 1, func: func_asin }),
        ("acos", Func { argc: 1, func: func_acos }),
        ("atan", Func { argc: 1, func: func_atan }),
        ("atan2", Func { argc: 2, func: func_atan2}),
        ("pow", Func { argc: 2, func: func_pow }),

        // 其他函数定义
    ]);
}

pub fn get_func(name:&str) -> Option<&Func> {
   HASH_MAP.get(name)
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
    pub(crate) fn new(tokens: &'a [Token]) -> Result<Self, CustomError> {
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
                ("^".to_string(), 50),
            ]),})
    }


    /// 解析表达式
    pub fn  parse_expression(&mut self) -> Result<Rc<AstNode>, CustomError> {
        self.depth+=1;
        let left = self.parse_primary();
        let right = self.parse_op(0,left?);
        self.depth-=1;
         right
    }

    pub fn parse_op(&mut self, priority: i8,mut left:  Rc<AstNode>)-> Result<Rc<AstNode>,CustomError>{
        loop{
        let op = self.curr_tok;
        match op {
            Some(token) => {
                match token.tok_type {
                    TokenType::OPERATOR => {
                        let cur_priority =  {
                            *self.priority_map.get(&token.tok).unwrap_or(&-1)
                        };
                        if cur_priority < priority {
                            return Ok(left);
                        }
                       self.next_token();
                        let mut right = self.parse_primary()?;

                        // 再次提前计算不可变借用
                        let next_priority = if let Some(next_token) = self.curr_tok {
                            *self.priority_map.get(&next_token.tok).unwrap_or(&0)
                        } else {
                            0
                        };

                        if cur_priority < next_priority {
                            right = self.parse_op(cur_priority+1,right)?;
                        }
                        left = Rc::from(AstNode::Binary(
                                BinaryAst {
                                    op: token.tok.clone(),
                                    left,
                                    right,
                                }
                            ))

                    },
                    TokenType::COMMA => {
                        return Ok(left)
                    },
                    _ => {
                        return Err(CustomError::InvalidSyntax)
                    }
                }
            },
            None => {
                return Ok(left)
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
                            val: token.tok.parse::<f64>().unwrap(),
                        };
                        self.next_token();
                        Ok(Rc::new(AstNode::Number(num)))
                    },
                    TokenType::IDENTIFIER => {
                        if !HASH_MAP.contains_key(&token.tok.as_str()){
                            Err(CustomError::FuncNotExist(token.tok.to_string()))?
                        }
                        let f=  self.parse_fn();
                        f
                    },
                    TokenType::OPERATOR => {
                        if token.tok == "("{
                            self.next_token();
                            let e = self.parse_expression();
                            if self.curr_tok.unwrap().tok == ")"{
                                self.next_token();
                                Ok(e?)
                            }  else{
                                return Err(CustomError::InvalidSyntax)
                            }
                        }else if token.tok == "-"{
                            self.next_token();
                            match self.curr_tok {
                                Some(token) => {
                                    match token.tok_type {
                                        TokenType::LITERAL => {
                                            let num = NumberAst {
                                                val: -token.tok.parse::<f64>().unwrap(),
                                            };
                                            self.next_token();
                                            Ok(Rc::new(AstNode::Number(num)))
                                        },
                                        _ => {
                                            Err(CustomError::InvalidSyntax)
                                        }
                                    }
                                }
                                None => {
                                    Err(CustomError::InvalidSyntax)
                                }
                            }

                        }
                        else{
                            Err(CustomError::InvalidSyntax)
                        }
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

    fn parse_fn(&mut self)-> Result<Rc<AstNode>,CustomError>{
        match  self.curr_tok{
            Some(token) => {
                match token.tok_type {
                    TokenType::IDENTIFIER => {
                        let mut f = FunCallerAST{
                            name: token.tok.to_string(),
                            arg: vec![],
                        };
                        self.next_token();
                        match self.curr_tok {
                            Some(token) => {
                                match token.tok_type {
                                    TokenType::OPERATOR => {
                                        if token.tok == "("{
                                            self.next_token();
                                                match self.curr_tok {
                                                    Some(token) => {
                                                        let e = self.parse_expression();
                                                        f.arg.push(e?);
                                                        loop  {
                                                            let token = self.curr_tok.unwrap();
                                                            match token.tok_type {
                                                                TokenType::COMMA => {
                                                                    self.next_token();
                                                                    continue
                                                                },
                                                                TokenType::OPERATOR => {
                                                                    if token.tok == ")"{
                                                                        self.next_token();
                                                                        return Ok(Rc::new(AstNode::FunCaller(f)))
                                                                    }else{
                                                                        return Err(CustomError::InvalidSyntax)
                                                                    }
                                                                }
                                                                _ => {
                                                                    f.arg.push(self.parse_expression()?);
                                                                }
                                                            }
                                                        }
                                                    },
                                                    None => {
                                                        return Err(CustomError::InvalidSyntax)
                                                    }

                                                }
                                        }
                                        return Ok(Rc::new(AstNode::FunCaller(f)))
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Err(CustomError::InvalidSyntax)
    }


    fn next_token(&mut self) -> bool{
        self.curr_idx += 1;
        if self.curr_idx > self.tokens.len() {
           false
        } else {
            self.curr_tok = self.tokens.get(self.curr_idx);
            true
        }
    }
}
