use std::rc::Rc;
use crate::ast::{get_func, AstNode, NumberAst};


pub fn exec(ast_node: &AstNode) -> f64 {
    match ast_node {
      AstNode::Number(num) => {
            println!("exec1 {}",num.val);
            return num.val;
        }
        AstNode::Binary(binary) => {
            let left =  binary.left.as_ref();
            let right =  binary.right.as_ref();
            println!("-----ast binary----- {:?} {:?}", binary, binary.op);
            match binary.op.as_str() {
                "+" => {
                    return exec(left) + exec(right);
                }
                "-" => {
                    return exec(left) - exec(right);
                }
                "*" => {
                    println!("--*-- {:?} {:?}", left, right);
                    return exec(left) * exec(right);
                }
                "/" => {
                    return exec(left) / exec(right);
                }
                "%" => {
                    return exec(left) % exec(right);
                }
                _ =>{
                    panic!("invalid operator");
                }
            }
        }
        AstNode::FunCaller(fun) => {
            let func = get_func(&fun.name);
            println!("---func--- {:?}", func);
            match func {
                Some(func) => {
                    let mut args: Vec<f64> = vec![];
                    println!("---args--- {:?}", fun.arg);
                   fun.arg.as_slice().as_ref().iter().for_each(|arg| {
                       let t= arg.as_ref();
                       match t {
                           AstNode::Number(num) => {
                               args.push(num.val);
                           }
                           AstNode::Binary(binary)=>{
                               println!("---binary---{:?}", binary);
                               let tmp = exec(arg);
                               println!("---tmp---{}",tmp);
                               args.push(tmp);
                           }

                           AstNode::FunCaller(fun)=>{
                               let tmp = exec(arg);
                               args.push(tmp);
                           }
                       }

                   });
                    let val = (func.func)(args.as_slice());
                    println!("---val---{}",val);
                    val
                }
                None => {
                    println!("func not exist {}",fun.name);
                    0.0
                }
            }
        }
        _ => {
            println!("invalid ast node");
            0.0
        }
    }
    }
