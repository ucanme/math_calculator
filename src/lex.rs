use std::fmt::Error;
use crate::error::error;
use crate::error::error::CustomError;
#[derive(Debug)]
pub enum TokenType {
    IDENTIFIER,
    LITERAL,
    OPERATOR,
    COMMA,
}
#[derive(Debug)]
pub struct  Token  {
    // raw characters
    pub(crate) tok: String,
    // type with Literal/Operator
    pub(crate) tok_type: TokenType,
    flag : i8,
    offset: i16
}


pub struct Parser {
   pub source: String,
   pub ch: char,
   pub offset: i16,
   pub error: Error
}
impl Parser {
    //noinspection RsBorrowChecker
    pub fn parse(mut self) -> Result<Vec<Token>, CustomError>{
        let mut vec:Vec<Token> = Vec::new();
        loop {
            let tok = self.next_tok();
            match tok {
                Ok(tok) => {
                    vec.push(tok);
                }
                Err(e) =>{
                    match e {
                        CustomError::EOF => {
                            return Ok(vec)
                        }
                        e=>{
                            return Err(e)
                        }
                    }
                }
            }
        }
    }
    fn next_tok(&mut self) -> Result<Token,CustomError> {
        if  self.offset >= self.source.len() as i16 {
            Err(error::CustomError::EOF)?
        }
        loop {
           if Self::is_whitespace(self.ch){
               self.next_ch()?;
               continue
           }
            break
        }

        let start = self.offset;

        if self.ch.is_digit(10){
            let mut num : u64 = self.ch.to_digit(10).unwrap() as u64;
            let mut num_str = self.ch.to_string();
            while self.next_ch().is_ok() && self.ch.is_digit(10) {
                let tmp : u64 = self.ch.to_digit(10).unwrap() as u64;
                num = num*10 + tmp;
                num_str.push(self.ch);
            }
            let mut fval: f64=  num as f64;
            if self.ch == '.'{
                num_str.push(self.ch);
                let mut i:u64 = 10;
                while self.next_ch().is_ok() && self.ch.is_digit(10) {
                    num_str.push(self.ch);
                    let tmp : u64 = self.ch.to_digit(10).unwrap() as u64;
                    fval = fval+(tmp as f64/i as f64) as f64;
                    i*=10
                }
            }


            //将fval计算为小数

            return Ok(Token{
                tok: num_str.to_string(),
                tok_type: TokenType::LITERAL,
                flag: 0,
                offset: start,
            })
        }

        if self.ch.is_ascii_alphabetic(){
            let mut tok = String::new();
            tok.push(self.ch);
            while self.next_ch().is_ok() &&  self.ch.is_ascii_lowercase() {
                tok.push(self.ch);
            }
            return Ok(Token{
                tok: tok,
                tok_type: TokenType::IDENTIFIER,
                flag: 0,
                offset: start,
            })
        }

        if self.ch == '+' || self.ch == '-' || self.ch == '*' || self.ch == '%' || self.ch == '/' {
            let tok = Token{
                tok: self.ch.to_string(),
                tok_type: TokenType::OPERATOR,
                flag: 0,
                offset: self.offset,
            };
            self.next_ch()?;
            return Ok(tok)
        }
        if self.ch == '(' || self.ch == ')' {
            let tok =  Ok(Token{
                tok: self.ch.to_string(),
                tok_type: TokenType::OPERATOR,
                flag: 0,
                offset: start,
            });
            let _ = self.next_ch();
            return tok
        }

        if self.ch == ',' {
            let tok =  Ok(Token{
                tok: self.ch.to_string(),
                tok_type: TokenType::COMMA,
                flag: 0,
                offset: start,
            });
            let _ = self.next_ch();
            return tok
        }


        Err(error::CustomError::UnknowChar(self.ch.to_string()))
    }
    fn is_whitespace(ch: char) -> bool {
        ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
    }

    fn is_digit_number(ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn next_ch(&mut self) -> Result<(),CustomError> {
        self.offset = self.offset+1;
        if self.offset >= self.source.len() as i16  {
             Err(CustomError::EOF)
        } else if self.offset < self.source.len() as i16 {
            self.ch = self.source.as_bytes()[self.offset as usize] as char;
            Ok(())
        }else{
            Err(CustomError::InvalidOffset)
        }
    }
}