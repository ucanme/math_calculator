use std::collections::HashMap;
use std::f32::consts::E;
use std::fmt::Alignment::Left;
use std::ptr::hash;
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
pub struct Func {
    argc: i8,
    func:  fn(args: &[AstNode]) -> f64
}


fn func_min(args: &[AstNode])->f64{
    return 0.0;
}

lazy_static::lazy_static! {
    static ref HASH_MAP: HashMap<&'static str, Func> = HashMap::from([
        ("min", Func { argc: 1, func: func_min }),
        // 其他函数定义
    ]);
}

fn get_func(name:&str) -> Option<&Func> {
   HASH_MAP.get(name)
}



// defFunc = map[string]defS{
// "sin": {1, defSin},
// "cos": {1, defCos},
// "tan": {1, defTan},
// "cot": {1, defCot},
// "sec": {1, defSec},
// "csc": {1, defCsc},
//
// "abs":   {1, defAbs},
// "ceil":  {1, defCeil},
// "floor": {1, defFloor},
// "round": {1, defRound},
// "sqrt":  {1, defSqrt},
// "cbrt":  {1, defCbrt},
//
// "noerr": {1, defNoerr},
//
// "max": {-1, defMax},
// "min": {-1, defMin},
// }
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
    pub fn  parse_expression(&mut self) -> Result<Rc<AstNode>, CustomError> {
        self.depth+=1;
        let left = self.parse_primary();
        print!("left: {:?}",left);
        let right = self.parse_op(0,left?);
        println!("right {:?}", right);
        right
    }

    pub fn parse_op(&mut self, priority: i8,mut left:  Rc<AstNode>)-> Result<Rc<AstNode>,CustomError>{
        loop{
        let op = self.curr_tok;
            println!("left {:?}", left);
            println!("op {:?}", op);
        match op {
            Some(token) => {
                match token.tok_type {
                    TokenType::OPERATOR => {
                        let cur_priority =  {
                            *self.priority_map.get(&token.tok).unwrap_or(&-1)
                        };

                        println!("cur:{:?} {:?}", cur_priority, priority);
                        if cur_priority < priority {
                            return Ok(left);
                        }
                        self.curr_idx += 1;
                        self.curr_tok = self.tokens.get(self.curr_idx);
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
                                    left: left,
                                    right: right,
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
                            val: token.tok.parse::<i64>().unwrap(),
                        };
                        self.curr_idx += 1;
                        self.curr_tok = self.tokens.get(self.curr_idx);
                        Ok(Rc::new(AstNode::Number(num)))
                    },
                    TokenType::IDENTIFIER => {
                        let f=  self.parse_fn();
                        println!("---f--- {:?}", f);
                        f
                    },
                    TokenType::OPERATOR => {
                        if token.tok == "("{
                            println!("parse_expression {:?}", token);
                            self.curr_idx += 1;
                            self.curr_tok = self.tokens.get(self.curr_idx);
                            let e = self.parse_expression();
                            println!("e {:?} {:?}", self.curr_tok, e);
                            println!("{}", self.curr_tok.unwrap().tok);
                            if self.curr_tok.unwrap().tok == ")"{
                                self.curr_idx += 1;
                                self.curr_tok = self.tokens.get(self.curr_idx);
                                print!("self.curr_tok {:?}", self.curr_tok);
                                Ok(e?)
                            }else{
                                return Err(CustomError::InvalidSyntax)
                            }
                        }else{
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
        println!("parse_fn {:?}", self.curr_tok);
        match  self.curr_tok{
            Some(token) => {
                match token.tok_type {
                    TokenType::IDENTIFIER => {
                        let mut f = FunCallerAST{
                            name: token.tok.to_string(),
                            arg: vec![],
                        };
                        self.curr_idx += 1;
                        self.curr_tok = self.tokens.get(self.curr_idx);
                        match self.curr_tok {
                            Some(token) => {

                                match token.tok_type {
                                    TokenType::OPERATOR => {
                                        println!("token {:?}", token);
                                        if token.tok == "("{
                                            self.curr_idx += 1;
                                            self.curr_tok = self.tokens.get(self.curr_idx);
                                            println!("AAA---- {:?}", self.curr_tok);
                                                match self.curr_tok {
                                                    Some(token) => {
                                                        let e = self.parse_expression();
                                                        println!("---e-- {:?}", e);
                                                        f.arg.push(e?);
                                                        self.curr_idx += 1;
                                                        self.curr_tok = self.tokens.get(self.curr_idx);
                                                        loop {
                                                            println!("---self.curr_tok {:?}", self.curr_tok);
                                                            let token = self.curr_tok.unwrap();
                                                            match token.tok_type {
                                                                TokenType::COMMA => {
                                                                    continue
                                                                },
                                                                TokenType::OPERATOR => {
                                                                    println!("----sss----");
                                                                    if token.tok == ")"{
                                                                        self.curr_idx += 1;
                                                                        self.curr_tok = self.tokens.get(self.curr_idx);
                                                                        return Ok(Rc::new(AstNode::FunCaller(f)))
                                                                    }else{
                                                                        return Err(CustomError::InvalidSyntax)
                                                                    }
                                                                }
                                                                _ => {
                                                                    f.arg.push(self.parse_expression()?);
                                                                    println!("parse_fn {:?}", f);

                                                                }
                                                            }
                                                        }
                                                    },
                                                    None => {
                                                        return Err(CustomError::InvalidSyntax)
                                                    }

                                                }
                                            println!("{}", self.curr_tok.unwrap().tok);
                                        }
                                        println!("{:?}", f);
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
        if self.curr_idx >= self.tokens.len() {
           false
        } else {
            self.curr_tok = self.tokens.get(self.curr_idx);
            true
        }
    }
}
